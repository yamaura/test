fn init_logging(outputs: Vec<Option<String>>, env: &str) {
    use tracing::subscriber::set_global_default;
    //use tracing_subscriber::{fmt, layer::SubscriberExt, EnvFilter};
    use tracing_subscriber::{layer::SubscriberExt, *};

    let default_env = if env.contains('=') {
        env.to_string()
    } else {
        format!(
            "{}={},{}={}",
            env!("CARGO_PKG_NAME").replace('-', "_"),
            env,
            env!("CARGO_BIN_NAME").replace('-', "_"),
            env
        )
    };

    fn file_appender(path: impl AsRef<str>) -> std::fs::File {
        std::fs::File::options()
            .append(true)
            .create(true)
            .open(path.as_ref())
            .unwrap_or_else(|e| panic!("{:?}: {}", path.as_ref(), e))
    }

    let t = tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::builder().parse(default_env).unwrap()),
        )
        .with_writer(std::io::stderr);

    match outputs.len() {
        0 => set_global_default(t.finish()),
        1 => match outputs[0].as_ref() {
            None => set_global_default(t.finish()),
            Some(p) => match p.as_str() {
                "journald" => {
                    set_global_default(Registry::default().with(tracing_journald::layer().unwrap()))
                }
                _ => set_global_default(t.with_writer(file_appender(p)).with_ansi(false).finish()),
            },
        },
        2 => match (outputs[0].as_ref(), outputs[1].as_ref()) {
            (None, Some(p)) => match p.as_str() {
                "journald" => {
                    set_global_default(t.finish().with(tracing_journald::layer().unwrap()))
                }
                _ => set_global_default(
                    t.finish().with(
                        fmt::Layer::default()
                            .with_writer(file_appender(p))
                            .with_ansi(false),
                    ),
                ),
            },
            _ => panic!("Invalid output"),
        },
        _ => panic!("Too many outputs"),
    }
    .unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //init_logging(vec![None, Some("log.log".to_string())], "debug");
    //    init_logging(vec![None, Some("journald".to_string())], "debug");
    init_logging(vec![Some("journald".to_string())], "debug");

    test::mylog();

    tracing::trace!("This is tracing::trace from main");
    tracing::debug!("This is tracing::debug from main");
    tracing::info!("This is tracing::info from main");
    tracing::warn!("This is tracing::warn from main");
    tracing::error!("This is tracing::error from main");

    log::trace!("This is log::trace from main");
    log::debug!("This is log::debug from main");
    log::info!("This is log::info from main");
    log::warn!("This is log::warn from main");
    log::error!("This is log::error from main");

    Ok(())
}
