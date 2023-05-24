use hex::FromHexError;

#[derive(Clone, Debug)]
pub enum UiError {
    Error(String),
}

impl From<FromHexError> for UiError {
    fn from(err: FromHexError) -> Self {
        UiError::Error(format!("Hex error: {}", err))
    }
}