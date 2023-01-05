use std::collections::HashMap;

use wgpu::{Device, TextureView, CommandEncoder, SurfaceTexture};
use winit::dpi::PhysicalSize;

use crate::Model;

pub struct Graphics {
    pub surface: wgpu::Surface,
    pub device: Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub render_pipeline: wgpu::RenderPipeline,
    pub models: HashMap<u32, Model>
}

impl Graphics {
    pub fn resize(&mut self, new_size:&PhysicalSize<u32>) {
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn begin(&self) -> (SurfaceTexture, TextureView, CommandEncoder) {
        let output = self.surface.get_current_texture().unwrap();
        let texture_view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        (output, texture_view, encoder)
    }

    pub fn end(&self, output:SurfaceTexture, encoder:CommandEncoder) {
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}