use thiserror::Error;
use ureq::Transport;

#[derive(Error, Debug)]
pub enum Error {
  // MATCHING HTTP ERROR CODE 400
  #[error("Invalid request, usually an input error - missing or invalid property, code: {0}")]
  InvalidRequest(u16),
  // MATCHING HTTP ERROR CODE 401
  #[error("Access token is missing or invalid, code: {0}")]
  UnauthorizedRequest(u16),
  // MATCHING HTTP ERROR CODE 402
  #[error("Not enough funds to perform the action, code: {0}")]
  InsufficientFunds(u16),
  // MATCHING HTTP ERROR CODE 403
  #[error("Can't perform the current action, code: {0}")]
  ForbiddenAction(u16),
  // MATCHING HTTP ERROR CODE 404
  #[error("Resource not found, code: {0}")]
  NotFound(u16),
  // MATCHING HTTP ERROR CODE 500
  #[error("Error on datacrunch's side, code: {0}")]
  ServerError(u16),
  // MATCHING HTTP ERROR CODE 503
  #[error("Not enough resources at the moment, try again later, code: {0}")]
  ServiceUnavailable(u16),
  #[error("Unkwon server error, code: {0}")]
  UnknowServerFailure(u16),
  #[error("Unkwon transport error, {0}")]
  UnknowTransportFailure(Transport),
  #[error("Unkwon transport error")]
  JsonParsing(#[source] std::io::Error),
  #[error("Uuid parsing error")]
  UuidParsing(#[source] uuid::Error),
}

impl From<ureq::Error> for Error {
  fn from(error: ureq::Error) -> Self {
    match error {
      ureq::Error::Transport(info) => Error::UnknowTransportFailure(info),
      ureq::Error::Status(code, _) => match code {
        400 => Error::InvalidRequest(code),
        401 => Error::UnauthorizedRequest(code),
        402 => Error::InsufficientFunds(code),
        403 => Error::ForbiddenAction(code),
        404 => Error::NotFound(code),
        500 => Error::ServerError(code),
        503 => Error::ServiceUnavailable(code),
        _ => Error::UnknowServerFailure(code),
      },
    }
  }
}
