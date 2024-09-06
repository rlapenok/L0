use std::error::Error;

use tracing::Level;
use tracing_subscriber::{
    filter::Directive,
    fmt::{
        format::{Compact, DefaultFields, Format},
        layer, Layer,
    },
    EnvFilter, Registry,
};

pub fn build_stdout_tracing_layer() -> Layer<Registry, DefaultFields, Format<Compact>> {
    layer()
        .compact()
        .with_level(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
}

pub fn build_env_filters(level: Level) -> Result<EnvFilter, Box<dyn Error>> {
    let filter = EnvFilter::from_default_env()
        .add_directive(Directive::from(level))
        .add_directive("sqlx=warn".parse()?);
    Ok(filter)
}
