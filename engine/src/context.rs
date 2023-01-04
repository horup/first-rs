use crate::Graphics;

pub struct Context<'a> {
    render:&'a mut Graphics
}


impl<'a> Context<'a> {
    pub fn new(render:&'a mut Graphics) -> Self {
        Self {
            render
        }
    }
    pub fn draw(&self) {
        let output = self.render.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.render.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
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

            render_pass.set_pipeline(&self.render.render_pipeline);
            render_pass.draw(0..6, 0..2);
        }

        self.render.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}