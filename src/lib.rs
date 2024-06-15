pub fn mylog() {
    tracing::trace!("This is tracing::trace from test::mylog");
    tracing::debug!("This is tracing::debug from test::mylog");
    tracing::info!("This is tracing::info from test::mylog");
    tracing::warn!("This is tracing::warn from test::mylog");
    tracing::error!("This is tracing::error from test::mylog");

    log::trace!("This is log::trace from test::mylog");
    log::debug!("This is log::debug from test::mylog");
    log::info!("This is log::info from test::mylog");
    log::warn!("This is log::warn from test::mylog");
    log::error!("This is log::error from test::mylog");
}
