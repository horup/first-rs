use wgpu::{Buffer, Device, BufferDescriptor};

pub struct Model {
    pub vertex_buffer:Buffer
}

impl Model {
    pub fn new(device:&Device) -> Self {
        let vertex_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: 1024,
            usage: wgpu::BufferUsages::VERTEX,
            mapped_at_creation: false,
        });

        Model {
            vertex_buffer
        }
    }
}