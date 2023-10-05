use wgpu::util::DeviceExt;

use crate::runtime::functions::scene_system::models::gltf::GltfAccessorType;

use super::gltf::{GltfAccessor, GltfBufferView, GltfMeshPrimitiveAttr};

#[derive(Debug, Default)]
pub struct SceneRenderData<'a> {
    pub vertexbuffer: Option<(wgpu::Buffer, wgpu::VertexBufferLayout<'a>)>,
    pub indexbuffer: Option<(wgpu::Buffer, wgpu::IndexFormat, u32)>, // len of index
}

impl SceneRenderData<'_> {
    pub fn process_primitive(
        &mut self,
        prim: &GltfMeshPrimitiveAttr,
        bufferviews: &[GltfBufferView],
        bin_data: &[u8],
        device: &wgpu::Device,
        accessor: &GltfAccessor,
    ) {
        match *prim {
            GltfMeshPrimitiveAttr::Position => {
                let (contents, stride, _) = process_buffer(bufferviews, bin_data, accessor);

                let vertexbuffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents,
                    usage: wgpu::BufferUsages::VERTEX,
                });
                let vertexbuffer_desc = wgpu::VertexBufferLayout {
                    array_stride: stride,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3],
                };

                self.vertexbuffer = Some((vertexbuffer, vertexbuffer_desc));
            }
            GltfMeshPrimitiveAttr::Normal => {}
            GltfMeshPrimitiveAttr::Tangent => todo!(),
            GltfMeshPrimitiveAttr::Weight => todo!(),
            GltfMeshPrimitiveAttr::Color => todo!(),
            GltfMeshPrimitiveAttr::MatrixPalette => todo!(),
            GltfMeshPrimitiveAttr::Joint => todo!(),
            GltfMeshPrimitiveAttr::TexCoord => todo!(),
        }
    }

    pub fn process_indices(
        &mut self,
        bufferviews: &[GltfBufferView],
        bin_data: &[u8],
        device: &wgpu::Device,
        accessor: &GltfAccessor,
    ) {
        assert_eq!(accessor.accessor_type, GltfAccessorType::Scalar);
        let (contents, _, count) = process_buffer(bufferviews, bin_data, accessor);

        let indexbuffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents,
            usage: wgpu::BufferUsages::INDEX,
        });
        self.indexbuffer = Some((indexbuffer, wgpu::IndexFormat::Uint32, count));
    }
}

fn process_buffer<'a>(
    bufferviews: &[GltfBufferView],
    buffers: &'a [u8],
    accessor: &GltfAccessor,
) -> (&'a [u8], u64, u32) {
    let bufferview_index = accessor.buffer_view.unwrap();
    let bufferview_offset = accessor.byte_offset.unwrap();
    let accessor_type = &accessor.accessor_type;
    let component_type = &accessor.component_type;
    let stride = accessor_type.to_length() * component_type.to_typesize();
    let count = accessor.count;
    let length = count * stride;

    let bufferview = &bufferviews[bufferview_index];
    let buffer_offset = bufferview.byte_offset.unwrap();
    let bufferview_length = bufferview.byte_length;
    let bufferview_data = &buffers[buffer_offset..(buffer_offset + bufferview_length)];

    let contents = &bufferview_data[bufferview_offset..(bufferview_offset + length)];

    return (contents, stride as _, count as _);
}
