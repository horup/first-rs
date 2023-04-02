pub fn smootherstep(edge0:f32, edge1:f32, x:f32) -> f32 {
    let x = (x - edge0) / (edge1 - edge0).clamp(0.0, 1.0);
    x * x * x * (x * (x * 6.0 - 15.0) + 10.0)
}