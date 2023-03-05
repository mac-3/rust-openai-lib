#[derive(Debug)]
pub enum Error {
    ApiKeyRequired,
    /// 401 - Invalid Authentication
    InvalidAuthentication,
    /// 401 - Incorrect API key provided
    IncorrectApiKeyProvided,
    /// 401
    OrganizationRequired,
    /// 429
    RateLimitExceeded,
    /// 429
    QuotaExceeded,
    /// 429
    EngineOverload,
    /// 500
    InternalServerError,
    ReqwestError(reqwest::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ApiKeyRequired => write!(f, "required api key is not specified"),
            Error::InvalidAuthentication => todo!(),
            Error::IncorrectApiKeyProvided => todo!(),
            Error::OrganizationRequired => todo!(),
            Error::RateLimitExceeded => todo!(),
            Error::QuotaExceeded => todo!(),
            Error::EngineOverload => todo!(),
            Error::InternalServerError => todo!(),
            Error::ReqwestError(e) => write!(f, "encountered error in reqwest library: {:?}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqwestError(value)
    }
}
