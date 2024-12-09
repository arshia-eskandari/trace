use error_stack::{ fmt::ColorMode, Report };
use owo_colors::OwoColorize;
use tracing_subscriber::EnvFilter;
pub use crate::feature::cli::init as cli_init;

struct Suggestion(&'static str);

pub fn error_reporting() {
    Report::set_color_mode(ColorMode::Color);
    Report::install_debug_hook::<Suggestion>(|Suggestion(value), context| {
        let body = format!("suggestion: {value}");
        match context.color_mode() {
            ColorMode::Color => context.push_body(body.green().to_string()),
            ColorMode::Emphasis => context.push_body(body.italic().to_string()),
            ColorMode::None => context.push_body(body),
        };
    });
}

pub fn tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    tracing_subscriber
        ::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(EnvFilter::builder().from_env_lossy())
        .with(ErrorLayer::default())
        .init()
}
