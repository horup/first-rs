use wgpu::{self, TextureView, RenderPipeline, RenderPass};

use crate::{Vertex, Graphics};

pub struct Model {
    pub vertex_buffer:wgpu::Buffer,
    pub num_vertices:u64
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
            mapped_at_creation: false,
        })
    }


    pub fn set_vertices(&mut self, device:&wgpu::Device, queue:&wgpu::Queue, vertices:&[Vertex]) {
        let size = vertices.len() as wgpu::BufferAddress * Vertex::size();
        if self.vertex_buffer.size() < size {
            self.vertex_buffer = Self::create_vertex_buffer(device, size);
        }

        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(vertices));

        self.num_vertices = vertices.len() as u64;
    }

    pub fn draw<'a>(&'a self, graphics:&Graphics) {
        if self.num_vertices == 0 {
            return;
        }

        let output = graphics.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = graphics.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&graphics.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..self.num_vertices as u32, 0..1);
        }

        graphics.queue.submit(std::iter::once(encoder.finish()));
        output.present();
     /*   render_pass.set_pipeline(&render_pipeline);
        let slice = self.vertex_buffer.slice(0..self.num_vertices);
        render_pass.set_vertex_buffer(0, slice);
        render_pass.draw(0..(self.num_vertices * 6) as u32, 0..1);*/
    }
}