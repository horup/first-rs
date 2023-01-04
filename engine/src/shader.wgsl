// Vertex shader

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};


@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    let verts = array(
        vec2(0., 0.),
        vec2(1., 1.),
        vec2(0., 1.),
        vec2(0., 0.),
        vec2(1., 0.),
        vec2(1., 1.),
    );

    let index = i32(in_vertex_index) % 6;

    var v = vec2(0., 0.);
    if (index == 0) {v = verts[0];}
    if (index == 1) {v = verts[1];}
    if (index == 2) {v = verts[2];}
    if (index == 3) {v = verts[3];}
    if (index == 4) {v = verts[4];}
    if (index == 5) {v = verts[5];}

    var x = v.x - 0.5; 
    var y = v.y - 0.5;

    var out: VertexOutput;
    let index = i32(in_vertex_index) % 3;

    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}