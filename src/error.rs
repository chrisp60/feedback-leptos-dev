#[derive(thiserror::Error, Debug)]
// Unless specified, use the underlying error message as MY error message
#[error(transparent)]
pub enum Error {
    // Attatches a formatted error message to my error message type.
    #[error("expected {var} to be set in environment, source: {source}")]
    EnvVariable {
        var: &'static str,
        source: std::env::VarError,
    },
    LeptosConfig(
        // Generates a From<LeptosConfigError> for crate::error::Error
        #[from] leptos::leptos_config::errors::LeptosConfigError,
    ),
    Io(#[from] std::io::Error),
}

// Type alias that has the Err variant of Result defaulting to my error type (crate::error::Error).
//
// This makes it so you can source this error from your code and skip specifying the error variant if
// you want. Setting a default type for the generic means it wont be a compile error if you specify
// a different type for the generic.
//
// See examples below.
pub type Result<T, E = Error> = std::result::Result<T, E>;

fn _compiles_with_a_different_error_type() -> Result<(), std::io::Error> {
    Ok(())
}

fn _compiles_with_my_error_type() -> Result<()> {
    Ok(())
}

/// Read the value of the env variable, or return a descriptive error.
pub fn env_or_error(var: &'static str) -> Result<String> {
    std::env::var(var).map_err(|source| Error::EnvVariable { var, source })
}
