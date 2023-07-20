mod editor;
mod runtime;

fn main() {
    let editor = editor::Editor::new();
    println!("Hello, world!");
    editor.run();
}
