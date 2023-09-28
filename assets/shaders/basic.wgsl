struct VertexInput {
    @location(0) position: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) vert_pos: vec3<f32>,
}

struct PushConstans {
    camera_mvp: mat4x4<f32>,
}

var<push_constant> constants: PushConstans;

@vertex
fn vs_main(
    in: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    // out.clip_position = camera * vec4<f32>(in.position, 1.0);
    out.clip_position = constants.camera_mvp * vec4<f32>(in.position, 1.0);
    out.vert_pos = out.clip_position.xyz;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}
