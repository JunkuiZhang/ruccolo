use std::path::PathBuf;

use wgpu::util::DeviceExt;

use crate::runtime::core::mathematics::Array4;

use self::{
    camera::CameraInfo,
    models::{gltf::GltfData, load},
};

pub mod camera;
pub mod models;

#[repr(C)]
#[derive(Debug)]
pub struct VerticesClip {
    pub vertex_buff: wgpu::Buffer,
    pub index_buff: wgpu::Buffer,
    pub indices_len: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, bytemuck::Zeroable, bytemuck::Pod)]
pub struct VertexInfo {
    pub vertex: [f32; 3],
    pub index: u32,
}

pub struct SceneManager {
    pub camera: CameraInfo,
    pub render_queue: Vec<VerticesClip>,
}

fn traverse_node(index: usize, gltf_data: &GltfData, bin_data: &[u8]) {
    let node = &gltf_data.nodes.as_ref().unwrap()[index];
    if let Some(child_list) = &node.children {
        for child in child_list {
            traverse_node(*child, gltf_data, bin_data);
        }
    }

    let buffer_views = gltf_data.buffer_views.as_ref().unwrap();
    if let Some(mesh_index) = node.mesh {
        let mesh = &gltf_data.meshes.as_ref().unwrap()[mesh_index];
        for mesh_element in mesh.primitives.iter() {
            for primitive_type in mesh_element.attributes.keys() {
                let primitive_index = mesh_element.attributes[primitive_type];
                match primitive_type {
                    models::gltf::GltfMeshPrimitiveAttr::Position => {
                        let accessor = &gltf_data.accessors.as_ref().unwrap()[primitive_index];
                        accessor.process(buffer_views, bin_data);

                        let Some(indices) = mesh_element.indices else {continue;};
                        let indices_data = &gltf_data.accessors.as_ref().unwrap()[indices];
                        indices_data.process(buffer_views, bin_data);
                    }
                    models::gltf::GltfMeshPrimitiveAttr::Normal => {}
                    models::gltf::GltfMeshPrimitiveAttr::Tangent => todo!(),
                    models::gltf::GltfMeshPrimitiveAttr::Weight => todo!(),
                    models::gltf::GltfMeshPrimitiveAttr::Color => todo!(),
                    models::gltf::GltfMeshPrimitiveAttr::MatrixPalette => todo!(),
                    models::gltf::GltfMeshPrimitiveAttr::Joint => todo!(),
                    models::gltf::GltfMeshPrimitiveAttr::TexCoord => todo!(),
                }
            }
            match mesh_element.mode.unwrap() {
                models::gltf::GltfMeshPrimitiveMode::Points => todo!(),
                models::gltf::GltfMeshPrimitiveMode::Lines => todo!(),
                models::gltf::GltfMeshPrimitiveMode::LineLoop => todo!(),
                models::gltf::GltfMeshPrimitiveMode::LineStrip => todo!(),
                models::gltf::GltfMeshPrimitiveMode::Triangles => {}
                models::gltf::GltfMeshPrimitiveMode::TriangleStrip => todo!(),
                models::gltf::GltfMeshPrimitiveMode::TriangleFan => todo!(),
            }
        }
    }
}

impl SceneManager {
    pub fn new() -> Self {
        SceneManager {
            camera: CameraInfo::default(),
            render_queue: Vec::new(),
        }
    }

    pub fn load_scene(&mut self, device: &wgpu::Device, bindgroup_list: &mut Vec<wgpu::BindGroup>) {
        // let scene_data = load("assets/scenes/triangle/tri.gltf");
        let (scene_data, bin_data) = load(
            "assets/scenes/CornellBox/scene.gltf",
            "assets/scenes/CornellBox/scene.bin",
        );
        // let scene_data = load("assets/scenes/Curtains/NewSponza.gltf");
        let default_scene = scene_data.default_scene.unwrap_or(0);
        for node_index in scene_data.scenes.as_ref().unwrap()[default_scene]
            .nodes
            .as_ref()
            .unwrap()
        {
            traverse_node(*node_index, &scene_data, &bin_data);
        }
        let bg_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bindgroup Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        // for model in vertices.iter() {
        //     let color_index = model.mesh.material_id.unwrap() as u32;
        //     let mut vertex_data = Vec::new();
        //     for chunk in model.mesh.positions.chunks_exact(3) {
        //         vertex_data.push(VertexInfo {
        //             vertex: [chunk[0], chunk[1], chunk[2]],
        //             index: color_index,
        //         });
        //     }
        //     let vertex_buff = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //         label: None,
        //         contents: bytemuck::cast_slice(&vertex_data),
        //         usage: wgpu::BufferUsages::VERTEX,
        //     });
        //     let index_buff = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //         label: None,
        //         contents: bytemuck::cast_slice(&model.mesh.indices),
        //         usage: wgpu::BufferUsages::INDEX,
        //     });
        //     let clip = VerticesClip {
        //         vertex_buff,
        //         index_buff,
        //         indices_len: model.mesh.indices.len() as u32,
        //     };
        //     self.render_queue.push(clip);
        // }

        // let mut color_data = Vec::new();
        // for material in materials.iter() {
        //     color_data.push(material.ambient.unwrap());
        // }
        // let color_buff = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        //     label: None,
        //     contents: bytemuck::cast_slice(&color_data),
        //     usage: wgpu::BufferUsages::STORAGE,
        // });

        // let bindgroup = device.create_bind_group(&wgpu::BindGroupDescriptor {
        //     label: Some("Bindgroup"),
        //     layout: (&bg_layout),
        //     entries: &[wgpu::BindGroupEntry {
        //         binding: 0,
        //         resource: wgpu::BindingResource::Buffer(color_buff.as_entire_buffer_binding()),
        //     }],
        // });
        // bindgroup_list.push(bindgroup);
    }
}
