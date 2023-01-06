use crate::{Engine, Model, Vertex};
use engine_sdk;

impl engine_sdk::Engine for Engine {
    fn define_texture(&mut self, id:u32, texture:String) {
        self.graphics.models.insert(id, Model::new(&self.graphics.device));
        dbg!(self.graphics.models.len());
    }

    fn draw_scene(&mut self, camera:&engine_sdk::Camera, scene:&engine_sdk::Scene) {
        let tex = 0;
        if let Some(model) = self.graphics.models.get_mut(&tex) {
            let mut vertices = Vec::with_capacity(scene.sprites.len() * 6);
            for sprite in scene.sprites.iter() {
                if sprite.tex == tex {
                    vertices.push(Vertex {
                        position: sprite.pos.into(),
                        color: [1.0, 1.0, 1.0, 1.0],
                    })
                }
            }
            model.set_vertices(&self.graphics.device, &self.graphics.queue, &vertices);
        }

        let (output, view, mut encoder) = self.graphics.begin();
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

            render_pass.set_pipeline(&self.graphics.render_pipeline);
            let size = 256*256;
            render_pass.draw(0..(size*6), 0..1);
        }

        self.graphics.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }

    fn frame_time(&self) -> std::time::Duration {
        self.diagnostics.frame_time
    }
}