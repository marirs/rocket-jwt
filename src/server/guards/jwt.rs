use rocket::{
    http::Status,
    outcome::{try_outcome, IntoOutcome},
    request::{FromRequest, Outcome, Request},
    State,
};

use crate::{backends::Backend, db::model::ApiKey, error::Error, secure::tokenizer::Tokenizer};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = Error;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let tokenizer = try_outcome!(request
            .guard::<&State<Tokenizer>>()
            .await
            .map_failure(|_| (Status::InternalServerError, Error::InternalError)));

        let backend = try_outcome!(request
            .guard::<&State<Backend>>()
            .await
            .map_failure(|_| (Status::InternalServerError, Error::InternalError)));

        request
            .headers()
            .get_one("Authorization")
            .map(|header| header.split("Bearer").collect::<Vec<_>>())
            .ok_or(Error::UnauthenticatedUser)
            .and_then(|bearer| {
                let token = bearer
                    .as_slice()
                    .get(1)
                    .map(|token| token.trim())
                    .unwrap_or_default();

                tokenizer.verify(token).map(|_| token.to_string())
            })
            .and_then(|token| match backend.find_user_by_token(&token) {
                Ok(user) if user.is_admin => Ok(ApiKey { token }),
                Ok(_) => Err(Error::ForbiddenAccess),
                Err(_) => Err(Error::UnauthenticatedUser),
            })
            .into_outcome(Status::BadRequest)
    }
}
