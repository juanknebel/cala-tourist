use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
  LoginFail,
  // -- Auth errors.
  AuthFailNoAuthTokenCookie,
  AuthFailTokenWrongFormat,
  AuthFailCtxNotInRequestExt,
  // -- Model errors.
  AttractionNotFound { id: i32 },
  // -- Similarity errors.
  GenerateSimilarityFail,
}

impl core::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{self:?}")
  }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
  fn into_response(self) -> Response {
    println!("->> {:<12} - {self:?}", "INTO_RES");
    let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

    // Insert the Error into the reponse.
    response.extensions_mut().insert(self);

    response
  }
}

impl Error {
  pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
    #[allow(unreachable_patterns)]
    match self {
      Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

      // -- Auth.
      Self::AuthFailNoAuthTokenCookie
      | Self::AuthFailTokenWrongFormat
      | Self::AuthFailCtxNotInRequestExt => {
        (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
      },

      // -- Model.
      Self::AttractionNotFound {
        ..
      } => (StatusCode::NOT_FOUND, ClientError::INVALID_PARAMS),

      // -- Similarity errors.
      Self::GenerateSimilarityFail => (
        StatusCode::INTERNAL_SERVER_ERROR,
        ClientError::SERVICE_ERROR,
      ),
      // -- Fallback.
      _ => (
        StatusCode::INTERNAL_SERVER_ERROR,
        ClientError::SERVICE_ERROR,
      ),
    }
  }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
  LOGIN_FAIL,
  NO_AUTH,
  INVALID_PARAMS,
  SERVICE_ERROR,
}
