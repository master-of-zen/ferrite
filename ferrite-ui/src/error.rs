use thiserror::Error;

#[derive(Error, Debug)]
pub enum UiError {
    #[error("Render error: {0}")]
    RenderError(String),

    #[error("Input error: {0}")]
    InputError(String),
}

pub type Result<T> = std::result::Result<T, UiError>;
