use wgpu::Device;
use winit::dpi::PhysicalSize;

pub struct Graphics {
    pub surface: wgpu::Surface,
    pub device: Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub render_pipeline: wgpu::RenderPipeline
}

impl Graphics {
    pub fn resize(&mut self, new_size:&PhysicalSize<u32>) {
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }
}