use wgpu::{Surface, Device, Queue, RenderPipeline};

pub struct Context<'a> {
    surface:&'a Surface,
    device:&'a Device,
    queue:&'a Queue,
    render_pipeline:&'a RenderPipeline
}


impl<'a> Context<'a> {
    pub fn new(surface:&'a Surface, device:&'a Device, queue:&'a Queue, render_pipeline:&'a RenderPipeline) -> Self {
        Self {
            surface,
            device,
            queue,
            render_pipeline
        }
    }
    pub fn draw(&self) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
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

            render_pass.set_pipeline(self.render_pipeline);
            render_pass.draw(0..3, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}