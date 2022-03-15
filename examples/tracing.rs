#[tokio::main]
async fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::{debug, error, event, info, instrument, span, trace, Level};
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    #[test]
    fn test_log_message() {
        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new(
                std::env::var("RUST_LOG").unwrap_or_else(|_| "error".into()),
            ))
            .with(tracing_subscriber::fmt::layer())
            .init();

        trace!("trace message");
        info!("info message");
        debug!("debug message");
        error!("error message");
    }

    #[test]
    fn test_fmt_formmatter() {
        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new(
                std::env::var("RUST_LOG").unwrap_or_else(|_| "error".into()),
            ))
            .with(tracing_subscriber::fmt::layer())
            .init();

        my_function_1("test param 1");
        my_function_2("test param 2");
    }

    #[instrument(
        level = "info", 
        name="function 1",     
        fields(
            the_param_is = %param
        )
    )]
    pub fn my_function_1(param: &str) {
        // This method will print something like:
        // DEBUG function 1{param="test param 1" the_param_is=test param 1}: tracing::tests: into the span
        debug!("into the span");
    }

    #[instrument(
        level = "info", 
        name="f2",    
        skip(param), 
        fields(
            the_param_is = %param
        )
    )]
    pub fn my_function_2(param: &str) {
        // This method will print something ike:
        // ERROR f2{the_param_is=test param 2}: tracing::tests: into the span
        error!("into the span");
    }

    #[test]
    pub fn test_bunyan_formmater() {

        // try to use the bunyan cli to format the message
        // cargo test --package axum_start --example tracing -- tests::test_bunyan_formmater | bunyan
        use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};

        let formatting_layer = BunyanFormattingLayer::new("tracing_example".to_string(), std::io::stdout);

        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new(
                std::env::var("RUST_LOG").unwrap_or_else(|_| "error".into()),
            ))
            .with(formatting_layer)
            .init();

            my_function_1("test param 1");
            my_function_2("test param 2");
    }
}
