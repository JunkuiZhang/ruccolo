struct VertexInput {
    @location(0) position: vec3<f32>,
    // @location(1) color_index: u32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color_index: u32,
}

struct PushConstans {
    camera_mvp: mat4x4<f32>,
}

struct Color {
    inner: vec3<f32>,
}

var<push_constant> constants: PushConstans;
// @group(0) @binding(0)
// var<storage, read> colors: array<Color>;

@vertex
fn vs_main(
    in: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    // out.clip_position = camera * vec4<f32>(in.position, 1.0);
    out.clip_position = constants.camera_mvp * vec4<f32>(in.position, 1.0);
    // out.color_index = in.color_index;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // return vec4<f32>(sqrt(colors[in.color_index].inner), 1.0);
    return vec4<f32>(0.5, 0.8, 0.3, 1.0);
}
