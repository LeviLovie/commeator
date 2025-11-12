use rocket::{
    data::{Data, FromData, Outcome, ToByteUnit},
    http::{ContentType, Status},
    request::Request,
    response::{Responder, Response, Result},
    tokio::io::AsyncReadExt,
};

const MAX_PROTOBUF_SIZE: usize = 2 * 1024 * 1024; // 1 MiB

use proto::prost::Message;

pub struct Proto<T>(pub T);

#[rocket::async_trait]
impl<'r, T> FromData<'r> for Proto<T>
where
    T: Message + Default,
{
    type Error = String;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        if req.content_type() != Some(&ContentType::new("application", "x-protobuf")) {
            return Outcome::Error((
                Status::UnsupportedMediaType,
                "expected application/x-protobuf".into(),
            ));
        }

        let mut buf = Vec::new();
        if data
            .open(MAX_PROTOBUF_SIZE.bytes())
            .read_to_end(&mut buf)
            .await
            .is_err()
        {
            return Outcome::Error((Status::InternalServerError, "failed to read".into()));
        }

        match T::decode(&*buf) {
            Ok(msg) => Outcome::Success(Proto(msg)),
            Err(e) => Outcome::Error((Status::BadRequest, e.to_string())),
        }
    }
}

pub struct ProtoResp<T>(pub T);

impl<'r, T> Responder<'r, 'static> for ProtoResp<T>
where
    T: Message,
{
    fn respond_to(self, _: &'r Request<'_>) -> Result<'static> {
        let mut buf = Vec::new();
        self.0
            .encode(&mut buf)
            .map_err(|_| Status::InternalServerError)?;

        Response::build()
            .header(ContentType::new("application", "x-protobuf"))
            .sized_body(buf.len(), std::io::Cursor::new(buf))
            .ok()
    }
}
