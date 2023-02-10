struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
    @location(2) uv: vec2<f32>
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
};

@vertex
fn vs_main(
    in: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = in.color;
    out.uv = in.uv;
    out.clip_position = camera.view_proj * vec4<f32>(in.position, 1.0);
    return out;
}

@group(1) @binding(0)
var diffuse_texture:texture_2d<f32>;

@group(1) @binding(1)
var diffuse_sampler:sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var pixel = textureSample(diffuse_texture, diffuse_sampler, in.uv) * in.color;
    if pixel[3] < 0.5 {
        discard;
    }
    return pixel;
}