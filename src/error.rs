use okapi::openapi3::Responses;
use rocket::{
    http::Status,
    request::Request,
    response::{self, Responder},
    serde::json::Json,
};
use rocket_okapi::{gen::OpenApiGenerator, response::OpenApiResponderInner};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Connection pool error: {0}")]
    PoolError(#[from] r2d2::Error),
    #[error("Backend error: {0}")]
    BackendError(#[from] diesel::result::Error),
    #[error("Format error: {0}")]
    FormatError(String),
    #[error("Launch failed: {0}")]
    RocketError(#[from] Box<rocket::Error>),
    #[error("Invalid access token: {0}")]
    JwtError(#[from] jwt_simple::Error),
    #[error("{0}")]
    BadRequest(String),
    #[error("Unauthenticated user")]
    UnauthenticatedUser,
    #[error("User does not have access rights")]
    ForbiddenAccess,
    #[error("Not found")]
    NotFound,
    #[error("Unknown route")]
    UnknownRoute,
    #[error("{0}")]
    InvalidResult(String),
    #[error("Internal error")]
    InternalError,
    #[error("Configuration Error")]
    ConfigurationError,
    #[error("Config file not found")]
    ConfigFileNotFound,
    #[error("Error getting ssl certificates")]
    SslCertificateError,
    #[error("Empty DB Url")]
    EmptyDBUrl,
    #[error("{0}")]
    Config(#[from] config::ConfigError),
    #[error("{0}")]
    ParseDuration(#[from] parse_duration::parse::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("Unknown Error")]
    Unknown,
}

impl Error {
    pub fn to_status(&self) -> Status {
        match *self {
            Self::UnauthenticatedUser => Status::Unauthorized,
            Self::ForbiddenAccess => Status::Forbidden,
            Self::BadRequest(_) | Self::JwtError(_) | Self::InvalidResult(_) => Status::BadRequest,
            Self::NotFound | Self::UnknownRoute => Status::NotFound,
            _ => Status::InternalServerError,
        }
    }
}

impl From<rocket::serde::json::Error<'_>> for Error {
    fn from(e: rocket::serde::json::Error<'_>) -> Self {
        Error::FormatError(format!("{:?}", e))
    }
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Error", 2)?;
        state.serialize_field("error", &self.to_string())?;
        state.serialize_field("code", &self.to_status().code)?;

        state.end()
    }
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        let status = self.to_status();

        response::Response::build_from(Json(self).respond_to(request)?)
            .status(status)
            .ok()
    }
}

impl OpenApiResponderInner for Error {
    fn responses(_generator: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
        Ok(Responses::default())
    }
}
