use thiserror::Error;

#[derive(Error, Debug)]
pub enum CrunchIOError {
    // MATCHING HTTP ERROR CODE 400
    #[error("Invalid request, usually an input error - missing or invalid property")]
    InvalidRequest,
    // MATCHING HTTP ERROR CODE 401
    #[error("Access token is missing or invalid")]
    UnauthorizedRequest,
    // MATCHING HTTP ERROR CODE 402
    #[error("Not enough funds to perform the action")]
    InsufficientFunds,
    // MATCHING HTTP ERROR CODE 403
    #[error("Can't perform the current action")]
    ForbiddenAction,
    // MATCHING HTTP ERROR CODE 404
    #[error("Resource not found")]
    NotFound,
    // MATCHING HTTP ERROR CODE 500
    #[error("Error on datacrunch's side")]
    ServerError,
    // MATCHING HTTP ERROR CODE 503
    #[error("Not enough resources at the moment, try again later")]
    ServiceUnavailable,
}
