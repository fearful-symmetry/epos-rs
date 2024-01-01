//! Error types.
use thiserror::Error;

use crate::status::Response;

#[derive(Error, Debug)]
pub enum EPOSError {
    #[error("error serializing XML")]
    SerializeError(#[from] quick_xml::DeError),
    #[error("Network request error")]
    NetworkError(#[from] reqwest::Error),
    #[error("invalid header")]
    InvalidHeaderError(#[from] reqwest::header::InvalidHeaderValue),
    #[error("error sending document: {status}")]
    ResponseError{status: Response}
}