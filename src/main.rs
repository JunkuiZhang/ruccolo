mod editor;
mod runtime;

fn main() {
    let mut logger = env_logger::Builder::from_default_env();
    logger.target(env_logger::Target::Stdout);
    logger.filter_level(log::LevelFilter::Info);
    logger.init();

    log::info!("Ruccolo Engine started.");
    let editor = editor::Editor::new();
    editor.run();
}
