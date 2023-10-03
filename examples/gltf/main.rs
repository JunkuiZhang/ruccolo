fn main() {
    let scene = gltf::Gltf::open("assets/scenes/triangle/tri.gltf").unwrap();

    println!("{:#?}", scene);
}
