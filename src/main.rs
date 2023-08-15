mod editor;
mod runtime;

#[cfg(any(feature = "profile-with-tracy"))]
fn main() {
    let mut logger = env_logger::Builder::from_default_env();
    logger.target(env_logger::Target::Stdout);
    logger.filter_level(log::LevelFilter::Info);
    logger.init();

    #[cfg(feature = "profile-with-tracy")]
    tracy_client::Client::start();

    profiling::register_thread!("Main Thread");
    log::info!("Ruccolo Engine started.");
    let editor = editor::Editor::new();
    editor.run();
}
