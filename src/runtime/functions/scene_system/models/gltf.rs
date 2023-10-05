use std::collections::BTreeMap;

/// https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GltfData {
    /// Names of glTF extensions used in this asset.
    #[serde(
        rename = "extensionsUsed",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub extensions_used: Vec<String>,
    /// Names of glTF extensions required to properly load this asset.
    #[serde(
        rename = "extensionsRequired",
        skip_serializing_if = "Vec::is_empty",
        default
    )]
    pub extensions_required: Vec<String>,
    /// An array of accessors.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub accessors: Vec<GltfAccessor>,
    // TODO: Animations
    /// Metadata about the glTF asset.
    pub asset: GltfAsset,
    ///	An array of buffers.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub buffers: Vec<GltfBuffer>,
    /// An array of bufferViews.
    #[serde(rename = "bufferViews", skip_serializing_if = "Vec::is_empty", default)]
    pub buffer_views: Vec<GltfBufferView>,
    // TODO: Cameras
    /// An array of images.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub images: Vec<GltfImage>,
    /// An array of materials.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub materials: Vec<GltfMaterial>,
    /// An array of meshes.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub meshes: Vec<GltfMesh>,
    /// An array of nodes.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub nodes: Vec<GltfNode>,
    /// An array of samplers.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub samplers: Vec<GltfSampler>,
    /// The index of the default scene.
    #[serde(
        rename = "scene",
        skip_serializing_if = "Option::is_none",
        default = "default_integer_zero"
    )]
    pub default_scene: Option<usize>,
    /// An array of scenes.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub scenes: Vec<GltfSceneElement>,
    // TODO: Skins
    /// An array of textures.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub textures: Vec<GltfTexture>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GltfAsset {
    /// A copyright message suitable for display to credit the content creator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,
    /// Tool that generated this glTF model. Useful for debugging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generator: Option<String>,
    /// The glTF version in the form of <major>.<minor> that this asset targets.
    pub version: String,
    #[serde(rename = "minVersion", skip_serializing_if = "Option::is_none")]
    pub min_version: Option<String>,
    /// Application-specific data.
    #[serde(flatten)]
    pub extras: GltfExtras,
}

/// The root nodes of a scene.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GltfSceneElement {
    /// The indices of each root node.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub nodes: Vec<usize>,
    /// The user-defined name of this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Application-specific data.
    #[serde(flatten)]
    pub extras: GltfExtras,
}

/// A node in the node hierarchy.
/// When the node contains `skin`, all `mesh.primitives` **MUST** contain `JOINTS_0` and `WEIGHTS_0` attributes.
/// A node **MAY** have either a `matrix` or any combination of `translation`/`rotation`/`scale` (TRS) properties.
/// TRS properties are converted to matrices and postmultiplied in the `T * R * S` order to compose
/// the `transformation` matrix;
/// first the scale is applied to the vertices, then the rotation, and then the translation.
/// If none are provided, the transform is the identity.
/// When a node is targeted for animation (referenced by an animation.channel.target), `matrix` **MUST NOT** be present.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GltfNode {
    /// The index of the camera referenced by this node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera: Option<usize>,
    /// The indices of this node’s children.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub children: Vec<usize>,
    /// The index of the skin referenced by this node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin: Option<usize>,
    /// A floating-point 4x4 transformation matrix stored in column-major order.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default = "default_matrix_identity"
    )]
    pub matrix: Option<[f32; 16]>,
    /// The index of the mesh in this node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh: Option<usize>,
    /// The node’s unit quaternion rotation in the order (x, y, z, w), where w is the scalar.
    #[serde(skip_serializing_if = "Option::is_none", default = "default_vector4")]
    pub rotation: Option<[f32; 4]>,
    /// The node’s non-uniform scale, given as the scaling factors along the x, y, and z axes.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default = "default_vector3_one"
    )]
    pub scale: Option<[f32; 3]>,
    /// The node’s translation along the x, y, and z axes.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default = "default_vector3_zero"
    )]
    pub translation: Option<[f32; 3]>,
    /// The weights of the instantiated morph target.
    /// The number of array elements **MUST** match the number of morph targets of the referenced mesh.
    /// When defined, `mesh` **MUST** also be defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weights: Option<Vec<f32>>,
    /// The user-defined name of this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Application-specific data.
    #[serde(flatten)]
    pub extras: GltfExtras,
}

/// A set of primitives to be rendered. Its global transform is defined by a node that references it.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GltfMesh {
    /// An array of primitives, each defining geometry to be rendered.
    pub primitives: Vec<GltfMeshPrimitive>,
    /// Array of weights to be applied to the morph targets.
    /// The number of array elements MUST match the number of morph targets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weights: Option<Vec<f32>>,
    /// The user-defined name of this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Application-specific data.
    #[serde(flatten)]
    pub extras: GltfExtras,
}

/// Geometry to be rendered with the given material.
/// Related WebGL functions: drawElements() and drawArrays()
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GltfMeshPrimitive {
    /// A plain JSON object, where each key corresponds to a mesh attribute semantic
    /// and each value is the index of the accessor containing attribute’s data.
    pub attributes: BTreeMap<GltfMeshPrimitiveAttr, usize>,
    /// The index of the accessor that contains the vertex indices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indices: Option<usize>,
    /// The index of the material to apply to this primitive when rendering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<usize>,
    /// The topology type of primitives to render.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub mode: Option<GltfMeshPrimitiveMode>,
    /// An array of morph targets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<serde_json::Value>,
    /// Application-specific data.
    #[serde(flatten)]
    pub extras: GltfExtras,
}

/// The topology type of primitives to render.
#[derive(
    serde_repr::Serialize_repr, serde_repr::Deserialize_repr, PartialEq, Debug, Default, Clone, Copy,
)]
#[repr(u32)]
pub enum GltfMeshPrimitiveMode {
    Points = 0,
    Lines = 1,
    LineLoop,
    LineStrip,
    #[default]
    Triangles,
    TriangleStrip,
    TriangleFan,
}

/// A plain JSON object, where each key corresponds to a mesh attribute semantic
/// and each value is the index of the accessor containing attribute’s data.
#[derive(
    serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy,
)]
pub enum GltfMeshPrimitiveAttr {
    #[serde(rename = "POSITION")]
    Position,
    #[serde(rename = "NORMAL")]
    Normal,
    #[serde(rename = "TANGENT")]
    Tangent,
    #[serde(rename = "WEIGHT")]
    Weight,
    #[serde(rename = "COLOR_0")]
    Color,
    #[serde(rename = "MATRIX_PALETTE")]
    MatrixPalette,
    #[serde(rename = "JOINT")]
    Joint,
    #[serde(rename = "TEXCOORD_0")]
    TexCoord,
    // TODO: Add more
}

/// The material appearance of a primitive.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GltfMaterial {
    /// The user-defined name of this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Application-specific data.
    #[serde(flatten)]
    pub extras: GltfExtras,
    /// A set of parameter values that are used to define the metallic-roughness material model
    /// from Physically Based Rendering (PBR) methodology.
    /// When undefined, all the default values of `pbrMetallicRoughness` MUST apply.
    #[serde(
        rename = "pbrMetallicRoughness",
        skip_serializing_if = "Option::is_none"
    )]
    pub pb_metallic_roughness: Option<GltfPbrMetallicRoughness>,
    /// The tangent space normal texture.
    #[serde(rename = "normalTexture", skip_serializing_if = "Option::is_none")]
    pub normal_texture: Option<GltfNormalTextureInfo>,
    /// The occlusion texture.
    #[serde(rename = "occlusionTexture", skip_serializing_if = "Option::is_none")]
    pub occlusion_texture: Option<GltfOcclusionTextureInfo>,
    /// The emissive texture.
    #[serde(rename = "emissiveTexture", skip_serializing_if = "Option::is_none")]
    pub emissive_texture: Option<GltfTextureInfo>,
    /// The factors for the emissive color of the material.
    #[serde(
        rename = "emissiveFactor",
        skip_serializing_if = "Option::is_none",
        default = "default_vector3_zero"
    )]
    pub emissive_factor: Option<[f32; 3]>,
    /// The alpha rendering mode of the material.
    #[serde(rename = "alphaMode", skip_serializing_if = "Option::is_none", default)]
    pub alpha_mode: Option<GltfMaterialAlphaMode>,
    /// The alpha cutoff value of the material.
    #[serde(
        rename = "alphaCutoff",
        skip_serializing_if = "Option::is_none",
        default = "default_float_half"
    )]
    pub alpha_cutoff: Option<f32>,
    /// Specifies whether the material is double sided.
    #[serde(
        rename = "doubleSided",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub double_sided: Option<bool>,
}

/// The material’s alpha rendering mode enumeration specifying the interpretation of the
/// alpha value of the base color.
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone, Copy)]
pub enum GltfMaterialAlphaMode {
    /// The alpha value is ignored, and the rendered output is fully opaque.
    #[default]
    #[serde(rename = "OPAQUE")]
    Opaque,
    /// The rendered output is either fully opaque or fully transparent depending on the
    /// alpha value and the specified `alphaCutoff` value;
    /// the exact appearance of the edges **MAY** be subject to implementation-specific
    /// techniques such as “Alpha-to-Coverage”.
    #[serde(rename = "MASK")]
    Mask,
    /// The alpha value is used to composite the source and destination areas.
    /// The rendered output is combined with the background using the normal painting operation
    /// (i.e. the Porter and Duff over operator).
    #[serde(rename = "BLEND")]
    Blend,
}

/// A set of parameter values that are used to define the metallic-roughness material model
/// from Physically-Based Rendering (PBR) methodology.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GltfPbrMetallicRoughness {
    /// The factors for the base color of the material.
    #[serde(
        rename = "baseColorFactor",
        skip_serializing_if = "Option::is_none",
        default = "default_pbr_base_color_factor"
    )]
    pub base_color_factor: Option<[f32; 4]>,
    /// The base color texture.
    #[serde(rename = "baseColorTexture", skip_serializing_if = "Option::is_none")]
    pub base_color_texture: Option<GltfTextureInfo>,
    /// The factor for the metalness of the material.
    #[serde(
        rename = "metallicFactor",
        skip_serializing_if = "Option::is_none",
        default = "default_float_one"
    )]
    pub metallic_factor: Option<f32>,
    /// The factor for the roughness of the material.
    #[serde(
        rename = "roughnessFactor",
        skip_serializing_if = "Option::is_none",
        default = "default_float_one"
    )]
    pub roughness_factor: Option<f32>,
    /// The metallic-roughness texture.
    #[serde(
        rename = "metallicRoughnessTexture",
        skip_serializing_if = "Option::is_none"
    )]
    pub metallic_roughness_texture: Option<GltfTextureInfo>,
    /// Application-specific data.
    #[serde(flatten)]
    pub extras: GltfExtras,
}

/// Reference to a texture.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GltfTextureInfo {
    /// The index of the texture.
    pub index: usize,
    /// The set index of texture’s TEXCOORD attribute used for texture coordinate mapping.
    #[serde(
        rename = "texCoord",
        skip_serializing_if = "Option::is_none",
        default = "default_integer_zero"
    )]
    pub tex_coord: Option<usize>,
    /// Application-specific data.
    #[serde(flatten)]
    pub extras: GltfExtras,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GltfAccessor {
    /// The index of the bufferView.
    #[serde(
        rename = "bufferView",
        skip_serializing_if = "Option::is_none",
        default = "default_integer_zero"
    )]
    pub buffer_view: Option<usize>,
    /// The offset relative to the start of the buffer view in bytes.
    #[serde(
        rename = "byteOffset",
        skip_serializing_if = "Option::is_none",
        default = "default_integer_zero"
    )]
    pub byte_offset: Option<usize>,
    /// The datatype of the accessor’s components.
    #[serde(rename = "componentType")]
    pub component_type: GltfAccessorComponentType,
    /// Specifies whether integer data values are normalized before usage.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub normalized: Option<bool>,
    /// The number of elements referenced by this accessor.
    pub count: usize,
    /// Specifies if the accessor’s elements are scalars, vectors, or matrices.
    #[serde(rename = "type")]
    pub accessor_type: GltfAccessorType,
    /// Maximum value of each component in this accessor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<Vec<f32>>,
    /// Minimum value of each component in this accessor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<Vec<f32>>,
    /// Sparse storage of elements that deviate from their initialization value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sparse: Option<GltfAccessorSparse>,
    /// The user-defined name of this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Application-specific data.
    #[serde(flatten)]
    pub extras: GltfExtras,
}

/// The datatype of the accessor’s components.
/// UNSIGNED_INT type **MUST NOT** be used for any accessor that is not referenced by `mesh.primitive.indices`.
/// Related WebGL functions: `type` parameter of `vertexAttribPointer()`.
/// The corresponding typed arrays are `Int8Array`, `Uint8Array`, `Int16Array`, `Uint16Array`,
/// `Uint32Array`, and `Float32Array`.
#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Debug)]
#[repr(u32)]
pub enum GltfAccessorComponentType {
    Byte = 5120,
    UnsignedByte = 5121,
    Short = 5122,
    UnsignedShort = 5123,
    UnsignedInt = 5125,
    Float = 5126,
}

impl GltfAccessorComponentType {
    pub fn to_typesize(&self) -> usize {
        match *self {
            GltfAccessorComponentType::Byte => std::mem::size_of::<u8>(),
            GltfAccessorComponentType::UnsignedByte => std::mem::size_of::<u8>(),
            GltfAccessorComponentType::Short => std::mem::size_of::<i16>(),
            GltfAccessorComponentType::UnsignedShort => std::mem::size_of::<u16>(),
            GltfAccessorComponentType::UnsignedInt => std::mem::size_of::<u32>(),
            GltfAccessorComponentType::Float => std::mem::size_of::<f32>(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub enum GltfAccessorType {
    #[serde(rename = "SCALAR")]
    Scalar,
    #[serde(rename = "VEC2")]
    Vec2,
    #[serde(rename = "VEC3")]
    Vec3,
    #[serde(rename = "VEC4")]
    Vec4,
    #[serde(rename = "MAT2")]
    Mat2,
    #[serde(rename = "MAT3")]
    Mat3,
    #[serde(rename = "MAT4")]
    Mat4,
}

impl GltfAccessorType {
    pub fn to_length(&self) -> usize {
        match *self {
            GltfAccessorType::Scalar => 1,
            GltfAccessorType::Vec2 => 2,
            GltfAccessorType::Vec3 => 3,
            GltfAccessorType::Vec4 => 4,
            GltfAccessorType::Mat2 => 2 * 4,
            GltfAccessorType::Mat3 => 3 * 4,
            GltfAccessorType::Mat4 => 4 * 4,
        }
    }
}

/// A view into a buffer generally representing a subset of the buffer.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GltfBufferView {
    /// The index of the buffer.
    pub buffer: usize,
    /// The offset into the buffer in bytes.
    #[serde(
        rename = "byteOffset",
        skip_serializing_if = "Option::is_none",
        default = "default_integer_zero"
    )]
    pub byte_offset: Option<usize>,
    /// The length of the bufferView in bytes.
    #[serde(rename = "byteLength")]
    pub byte_length: usize,
    /// The stride, in bytes.
    #[serde(rename = "byteStride", skip_serializing_if = "Option::is_none")]
    pub byte_stride: Option<usize>,
    /// The hint representing the intended GPU buffer type to use with this buffer view.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<usize>,
    /// The user-defined name of this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Application-specific data.
    #[serde(flatten)]
    pub extras: GltfExtras,
}

/// A buffer points to binary geometry, animation, or skins.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GltfBuffer {
    /// The URI (or IRI) of the buffer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// The length of the buffer in bytes.
    #[serde(rename = "byteLength")]
    pub byte_length: usize,
    /// The user-defined name of this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Application-specific data.
    #[serde(flatten)]
    pub extras: GltfExtras,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GltfAccessorSparse {
    /// Number of deviating accessor values stored in the sparse array.
    pub count: usize,
    /// An object pointing to a buffer view containing the indices of deviating accessor values.
    /// The number of indices is equal to `count`. Indices MUST strictly increase.
    pub indices: GltfAccessorSparseIndices,
    /// An object pointing to a buffer view containing the deviating accessor values.
    pub values: GltfAccessorSparseValues,
    /// Application-specific data.
    #[serde(flatten)]
    pub extras: GltfExtras,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GltfAccessorSparseIndices {
    /// The index of the buffer view with sparse indices.
    /// The referenced buffer view MUST NOT have its `target` or `byteStride` properties defined.
    /// The buffer view and the optional `byteOffset` MUST be aligned to the `componentType` byte length.
    #[serde(rename = "bufferView")]
    pub buffer_view: usize,
    /// The offset relative to the start of the buffer view in bytes.
    #[serde(
        rename = "byteOffset",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub buffer_offset: Option<usize>,
    /// The indices data type.
    #[serde(rename = "componentType")]
    pub component_type: usize,
    /// Application-specific data.
    #[serde(flatten)]
    pub extras: GltfExtras,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GltfAccessorSparseValues {
    /// The index of the bufferView with sparse values.
    /// The referenced buffer view MUST NOT have its `target` or `byteStride` properties defined.
    #[serde(rename = "bufferView")]
    pub buffer_view: usize,
    /// The offset relative to the start of the buffer view in bytes.
    #[serde(
        rename = "byteOffset",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub buffer_offset: Option<usize>,
    /// Application-specific data.
    #[serde(flatten)]
    pub extras: GltfExtras,
}

/// Image data used to create a texture. Image MAY be referenced by an URI (or IRI) or a buffer view index.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GltfImage {
    /// The URI (or IRI) of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// The image’s media type. This field MUST be defined when `bufferView` is defined.
    #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// The index of the bufferView that contains the image.
    /// This field MUST NOT be defined when `uri` is defined.
    #[serde(rename = "bufferView", skip_serializing_if = "Option::is_none")]
    pub buffer_view: Option<usize>,
    /// The user-defined name of this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Application-specific data.
    #[serde(flatten)]
    pub extras: GltfExtras,
}

/// Texture sampler properties for filtering and wrapping modes.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GltfSampler {
    /// Magnification filter.
    #[serde(rename = "magFilter", skip_serializing_if = "Option::is_none")]
    pub mag_filter: Option<usize>,
    /// Minification filter.
    #[serde(rename = "minFilter", skip_serializing_if = "Option::is_none")]
    pub min_filter: Option<usize>,
    /// S (U) wrapping mode.
    #[serde(rename = "wrapS", skip_serializing_if = "Option::is_none")]
    pub wrap_s: Option<GltfSamplerWrap>,
    /// T (V) wrapping mode.
    #[serde(rename = "wrapT", skip_serializing_if = "Option::is_none")]
    pub wrap_t: Option<GltfSamplerWrap>,
    /// The user-defined name of this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Application-specific data.
    #[serde(flatten)]
    pub extras: GltfExtras,
}

/// S (U) wrapping mode. All valid values correspond to WebGL enums.
/// Related WebGL functions: `samplerParameteri()` with pname equal to `TEXTURE_WRAP_S`
/// Related WebGL functions: `samplerParameteri()` with pname equal to `TEXTURE_WRAP_T`
#[derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Debug, Default, Clone, Copy)]
#[repr(usize)]
pub enum GltfSamplerWrap {
    ClampToEdge = 33071,
    MirroredRepeat = 33648,
    #[default]
    Repeat = 10497,
}

/// A texture and its sampler.
/// Related WebGL functions: createTexture(), deleteTexture(), bindTexture(), texImage2D(),
/// and texParameterf()
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GltfTexture {
    /// The index of the sampler used by this texture.
    /// When undefined, a sampler with repeat wrapping and auto filtering SHOULD be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampler: Option<usize>,
    /// The index of the image used by this texture.
    /// When undefined, an extension or other mechanism SHOULD supply an alternate texture source,
    /// otherwise behavior is undefined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<usize>,
    /// The user-defined name of this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Application-specific data.
    #[serde(flatten)]
    pub extras: GltfExtras,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GltfNormalTextureInfo {
    /// The index of the texture.
    pub index: usize,
    /// The set index of texture’s TEXCOORD attribute used for texture coordinate mapping.
    #[serde(
        rename = "texCoord",
        skip_serializing_if = "Option::is_none",
        default = "default_integer_zero"
    )]
    pub tex_coord: Option<usize>,
    /// The scalar parameter applied to each normal vector of the normal texture.
    #[serde(skip_serializing_if = "Option::is_none", default = "default_float_one")]
    pub scale: Option<f32>,
    /// Application-specific data.
    #[serde(flatten)]
    pub extras: GltfExtras,
}

/// Reference to a texture.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GltfOcclusionTextureInfo {
    /// The index of the texture.
    pub index: usize,
    /// The set index of texture’s TEXCOORD attribute used for texture coordinate mapping.
    #[serde(
        rename = "texCoord",
        skip_serializing_if = "Option::is_none",
        default = "default_integer_zero"
    )]
    pub tex_coord: Option<usize>,
    /// The scalar parameter applied to each normal vector of the normal texture.
    #[serde(skip_serializing_if = "Option::is_none", default = "default_float_one")]
    pub strength: Option<f32>,
    /// Application-specific data.
    #[serde(flatten)]
    pub extras: GltfExtras,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GltfExtras {
    /// JSON object with extension-specific objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<serde_json::Value>,
    /// Application-specific data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<serde_json::Value>,
}

fn default_integer_zero() -> Option<usize> {
    Some(0)
}

fn default_float_one() -> Option<f32> {
    Some(1.0)
}

fn default_float_half() -> Option<f32> {
    Some(0.5)
}

fn default_vector3_zero() -> Option<[f32; 3]> {
    Some([0.0; 3])
}

fn default_vector3_one() -> Option<[f32; 3]> {
    Some([1.0; 3])
}

fn default_vector4() -> Option<[f32; 4]> {
    Some([0.0, 0.0, 0.0, 1.0])
}

fn default_pbr_base_color_factor() -> Option<[f32; 4]> {
    Some([1.0; 4])
}

fn default_matrix_identity() -> Option<[f32; 16]> {
    Some([
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ])
}
