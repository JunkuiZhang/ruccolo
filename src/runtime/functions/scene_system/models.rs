pub mod gltf;
pub mod renderable;

use std::{fmt::Debug, io::Read, path::Path};

// pub struct ModelLoader;
const BUFFER_SIZE: usize = 8 * 1024;

pub fn load<P: AsRef<Path> + Debug + Clone>(
    gltf_path: P,
    bin_path: P,
) -> (gltf::GltfData, Vec<u8>) {
    let gltf_file = std::fs::File::open(gltf_path.clone()).expect(&format!(
        "Unable to open the gltf file: {:?}",
        gltf_path.clone()
    ));
    let reader = std::io::BufReader::new(gltf_file);
    let content: gltf::GltfData = serde_json::from_reader(reader)
        .expect(&format!("Unable to parser json file: {:?}", gltf_path));

    let bin_length = content.buffers[0].byte_length;
    let bin_file = std::fs::File::open(bin_path.clone()).expect(&format!(
        "Unable to open the bin file: {:?}",
        bin_path.clone()
    ));
    let mut bin = std::io::BufReader::with_capacity(BUFFER_SIZE, bin_file);
    let bin_data = read_binary(&mut bin, bin_length);
    // println!("{:#?}", content.accessors.unwrap());
    return (content, bin_data);
}

fn read_binary(reader: &mut std::io::BufReader<std::fs::File>, length: usize) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::with_capacity(length);
    reader.read_to_end(&mut res).unwrap();
    log::info!("Read bytes: {}/{}", res.len(), length);
    return res;
}
