use std::{mem::size_of, ops::Range};

use wgpu::{self};

use crate::{Vertex, GraphicsContext};

pub struct Model {
    pub vertex_buffer:wgpu::Buffer,
    pub index_buffer:wgpu::Buffer,
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

    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indicies.clear();
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

    pub fn write(&mut self, graphics:&mut GraphicsContext) {
        let vertices_size = self.vertices.len() as wgpu::BufferAddress * Vertex::size();
        if self.vertex_buffer.size() < vertices_size {
            self.vertex_buffer = Self::create_vertex_buffer(graphics.device, vertices_size);
            
        }

        let indicies_size = self.indicies.len() as wgpu::BufferAddress * size_of::<u32>() as wgpu::BufferAddress;
        if self.index_buffer.size() < indicies_size {
            self.index_buffer = Self::create_index_buffer(graphics.device, vertices_size);
        }

        let slice:&[u8] = bytemuck::cast_slice(&self.vertices);
        graphics.queue.write_buffer(&self.vertex_buffer, 0, slice);

        let slice:&[u8] = bytemuck::cast_slice(&self.indicies);
        graphics.queue.write_buffer(&self.index_buffer, 0, slice);
    }

    pub fn draw<'a>(&'a self, graphics:&mut GraphicsContext, diffuse_texture:&crate::Texture) {
        self.draw_indexed(graphics, 0..self.indicies.len() as u32, diffuse_texture);
    }

    pub fn draw_indexed<'a>(&'a self, graphics:&mut GraphicsContext, indicies:Range<u32>, diffuse_texture:&crate::Texture) {
        if self.index_buffer.size() == 0 || self.indicies.len() == 0 {
            return;
        }

        {
            let mut render_pass = graphics.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &graphics.surface_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&graphics.render_pipeline);
            render_pass.set_bind_group(0, &graphics.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &diffuse_texture.texture_bind_group, &[]);
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw_indexed(indicies, 0, 0..1);
        }
    }
}