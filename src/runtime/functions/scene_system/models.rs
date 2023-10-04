pub mod gltf;

use std::{fmt::Debug, path::Path};

pub struct ModelLoader;

pub fn load<P: AsRef<Path> + Debug + Clone>(path: P) {
    let file = std::fs::File::open(path.clone())
        .expect(&format!("Unable to open the path: {:?}", path.clone()));
    let reader = std::io::BufReader::new(file);
    let content: gltf::GltfData =
        serde_json::from_reader(reader).expect(&format!("Unable to parser json file: {:?}", path));

    println!("{:#?}", content.accessors.unwrap());
}
