mod editor;
mod runtime;

fn main() {
    prepare();

    profiling::register_thread!("Main Thread");
    log::info!("Ruccolo Engine started.");
    let editor = editor::Editor::new();
    editor.run();
}

fn prepare() {
    #[cfg(feature = "profile-with-tracy")]
    profiling::tracy_client::Client::start();

    let mut logger = env_logger::Builder::from_default_env();
    logger.target(env_logger::Target::Stdout);
    logger.filter_level(log::LevelFilter::Info);
    logger.init();
}
