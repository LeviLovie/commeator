use anyhow::Error;
use rocket::{
    http::Status,
    response::{Responder, Response},
};

pub type ApiResult<T> = Result<T, ApiError>;

pub struct ApiError(pub Error);

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _req: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        println!("Error: {}", self.0);

        for cause in self.0.chain().skip(1) {
            eprintln!("  caused by: {}", cause);
        }

        Response::build().status(Status::InternalServerError).ok()
    }
}

impl From<Error> for ApiError {
    fn from(e: Error) -> Self {
        ApiError(e)
    }
}
