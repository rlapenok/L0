use tracing_subscriber::{
    fmt::{
        format::{Compact, DefaultFields, Format},
        layer, Layer,
    },
    Registry,
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