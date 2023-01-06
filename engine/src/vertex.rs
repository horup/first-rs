use std::mem::size_of;
use bytemuck;
use wgpu;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

impl Vertex {
    pub fn size() -> wgpu::BufferAddress {
        size_of::<Vertex>() as wgpu::BufferAddress
    }
}