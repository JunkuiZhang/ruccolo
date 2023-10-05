use self::{
    camera::CameraInfo,
    models::{gltf::GltfData, load, renderable::SceneRenderData},
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
}

fn traverse_node(
    index: usize,
    gltf_data: &GltfData,
    bin_data: &[u8],
    device: &wgpu::Device,
    render_queue: &mut Vec<SceneRenderData>,
) {
    let node = &gltf_data.nodes[index];
    for child in node.children.iter() {
        traverse_node(*child, gltf_data, bin_data, device, render_queue);
    }

    let buffer_views = &gltf_data.buffer_views;
    let Some(mesh_index) = node.mesh else { return; };
    let mesh = &gltf_data.meshes[mesh_index];
    for mesh_element in mesh.primitives.iter() {
        let mut render_data = SceneRenderData::default();
        for primitive_type in mesh_element.attributes.keys() {
            let primitive_index = mesh_element.attributes[primitive_type];
            let accessor = &gltf_data.accessors[primitive_index];
            render_data.process_primitive(primitive_type, buffer_views, bin_data, device, accessor);
        }
        let indices_index = mesh_element.indices.unwrap();
        let indices_data = &gltf_data.accessors[indices_index];
        render_data.process_indices(buffer_views, bin_data, device, indices_data);

        match mesh_element.mode.unwrap() {
            models::gltf::GltfMeshPrimitiveMode::Points => todo!(),
            models::gltf::GltfMeshPrimitiveMode::Lines => todo!(),
            models::gltf::GltfMeshPrimitiveMode::LineLoop => todo!(),
            models::gltf::GltfMeshPrimitiveMode::LineStrip => todo!(),
            models::gltf::GltfMeshPrimitiveMode::Triangles => {}
            models::gltf::GltfMeshPrimitiveMode::TriangleStrip => todo!(),
            models::gltf::GltfMeshPrimitiveMode::TriangleFan => todo!(),
        }

        render_queue.push(render_data);
    }
}

impl SceneManager {
    pub fn new() -> Self {
        SceneManager {
            camera: CameraInfo::default(),
        }
    }

    pub fn load_scene(
        &mut self,
        device: &wgpu::Device,
        render_queue: &mut Vec<SceneRenderData>,
        bindgroup_list: &mut Vec<wgpu::BindGroup>,
    ) {
        // let scene_data = load("assets/scenes/triangle/tri.gltf");
        let (scene_data, bin_data) = load(
            "assets/scenes/CornellBox/scene.gltf",
            "assets/scenes/CornellBox/scene.bin",
        );
        // let scene_data = load("assets/scenes/Curtains/NewSponza.gltf");
        let default_scene = scene_data.default_scene.unwrap();
        for node_index in scene_data.scenes[default_scene].nodes.iter() {
            traverse_node(*node_index, &scene_data, &bin_data, device, render_queue);
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
