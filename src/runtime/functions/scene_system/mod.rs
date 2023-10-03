use std::path::PathBuf;

use wgpu::util::DeviceExt;

use crate::runtime::core::mathematics::Array4;

use self::{camera::CameraInfo, models::load};

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

impl SceneManager {
    pub fn new() -> Self {
        SceneManager {
            camera: CameraInfo::default(),
            render_queue: Vec::new(),
        }
    }

    pub fn load_scene(&mut self, device: &wgpu::Device, bindgroup_list: &mut Vec<wgpu::BindGroup>) {
        let path = PathBuf::new();
        let path = path
            .join("assets")
            .join("scenes")
            .join("CornellBox-Original")
            .join("CornellBox-Original.obj");
        // let (vertices, materials) = load(path);
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
