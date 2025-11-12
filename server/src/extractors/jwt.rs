use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};

use crate::jwt;

pub struct Jwt(pub jwt::Claims);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Jwt {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let header = req.headers().get_one("Authorization");

        match header {
            Some(hdr) => {
                if hdr.starts_with("Bearer ") {
                    let token = hdr.trim_start_matches("Bearer ").trim().to_string();

                    match jwt::verify(token.clone()) {
                        jwt::JwtStatus::Valid(claims) => {
                            return Outcome::Success(Jwt(claims));
                        }
                        jwt::JwtStatus::Expired => {
                            return Outcome::Error((Status::Unauthorized, "token expired".into()));
                        }
                        jwt::JwtStatus::Invalid => {
                            return Outcome::Error((Status::Unauthorized, "invalid token".into()));
                        }
                    }
                } else {
                    Outcome::Error((Status::Unauthorized, "invalid authorization header".into()))
                }
            }
            _ => Outcome::Error((Status::Unauthorized, "no header found".into())),
        }
    }
}
