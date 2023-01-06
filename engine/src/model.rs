use wgpu;

use crate::Vertex;

pub struct Model {
    pub vertex_buffer:wgpu::Buffer,
    pub num_vertices:u32
}

impl Model {
    pub fn new(device:&wgpu::Device) -> Self {
        let vertex_buffer = Self::create_vertex_buffer(device, 0);

        Model {
            vertex_buffer,
            num_vertices:0
        }
    }

    fn create_vertex_buffer(device:&wgpu::Device, size:u64) -> wgpu::Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: size,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: true,
        })
    }


    pub fn set_vertices(&mut self, device:&wgpu::Device, queue:&wgpu::Queue, vertices:&[Vertex]) {
        let size = vertices.len() as wgpu::BufferAddress * Vertex::size();
        if self.vertex_buffer.size() < size {
            self.vertex_buffer = Self::create_vertex_buffer(device, size);
        }

        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(vertices));

        self.num_vertices = vertices.len() as u32;
    }
}