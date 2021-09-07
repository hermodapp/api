//! Contains HTTP Handlers that directly receive and respond to requests to the server.
mod auth;
mod health_check;
mod index;

pub use auth::*;
pub use health_check::*;
pub use index::*;

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}
