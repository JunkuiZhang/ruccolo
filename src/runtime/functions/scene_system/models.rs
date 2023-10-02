use std::path::Path;

use tobj::{Material, Model};

pub struct Vertex;

pub fn load<P: AsRef<Path>>(path: P) -> (Vec<Model>, Vec<Material>) {
    let (mods, mats) = tobj::load_obj(
        path.as_ref(),
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ignore_points: false,
            ignore_lines: false,
        },
    )
    .unwrap();
    let mats = mats.unwrap();

    return (mods, mats);
}
