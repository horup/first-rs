// Vertex shader

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};


@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    let s = 0.01;
    let verts = array(
        vec2(0., 0.),
        vec2(s, s),
        vec2(0., s),
        vec2(0., 0.),
        vec2(s, 0.),
        vec2(s, s),
    );

    let index = i32(in_vertex_index) % 6;

    var v = vec2(0., 0.);
    if (index == 0) {v = verts[0];}
    if (index == 1) {v = verts[1];}
    if (index == 2) {v = verts[2];}
    if (index == 3) {v = verts[3];}
    if (index == 4) {v = verts[4];}
    if (index == 5) {v = verts[5];}

    var xy = f32(i32(f32(in_vertex_index) / 6.0)) * s;
    var x = v.x - s/2.0 + xy; 
    var y = v.y - s/2.0;

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