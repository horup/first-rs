use std::mem::size_of;

use wgpu::{self};

use crate::{Vertex, Graphics};

pub struct Model {
    vertex_buffer:wgpu::Buffer,
    index_buffer:wgpu::Buffer,
    pub vertices:Vec<Vertex>,
    pub indicies:Vec<u32>
}

impl Model {
    pub fn new(device:&wgpu::Device) -> Self {
        let vertex_buffer = Self::create_vertex_buffer(device, 0);
        let index_buffer = Self::create_index_buffer(device, 0);

        Model {
            vertex_buffer,
            index_buffer,
            vertices:Vec::new(),
            indicies:Vec::new()
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

    fn create_index_buffer(device:&wgpu::Device, size:u64) -> wgpu::Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Index Buffer"),
            size: size,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })

    }

    pub fn write(&mut self, graphics:&Graphics) {
        let vertices_size = self.vertices.len() as wgpu::BufferAddress * Vertex::size();
        if self.vertex_buffer.size() < vertices_size {
            self.vertex_buffer = Self::create_vertex_buffer(&graphics.device, vertices_size);
            
        }

        let indicies_size = self.indicies.len() as wgpu::BufferAddress * size_of::<u32>() as wgpu::BufferAddress;
        if self.index_buffer.size() < indicies_size {
            self.index_buffer = Self::create_index_buffer(&graphics.device, vertices_size);
        }

        let slice:&[u8] = bytemuck::cast_slice(&self.vertices);
        graphics.queue.write_buffer(&self.vertex_buffer, 0, slice);

        let slice:&[u8] = bytemuck::cast_slice(&self.indicies);
        graphics.queue.write_buffer(&self.index_buffer, 0, slice);
    }

    pub fn draw<'a>(&'a self, graphics:&Graphics) {
        if self.index_buffer.size() == 0 || self.indicies.len() == 0 {
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
            render_pass.set_bind_group(0, &graphics.camera_bind_group, &[]);
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw_indexed(0..self.indicies.len() as u32, 0, 0..1);
        }

        graphics.queue.submit(std::iter::once(encoder.finish()));
        output.present();
     /*   render_pass.set_pipeline(&render_pipeline);
        let slice = self.vertex_buffer.slice(0..self.num_vertices);
        render_pass.set_vertex_buffer(0, slice);
        render_pass.draw(0..(self.num_vertices * 6) as u32, 0..1);*/
    }
}