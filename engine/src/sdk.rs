use crate::{Engine, Model, Vertex};
use engine_sdk::{self, glam::Vec3};

impl engine_sdk::Engine for Engine {
    fn define_texture(&mut self, id:u32, texture:String) {
        self.models.insert(id, Model::new(&self.graphics.device));
        dbg!(self.models.len());
    }

    fn draw_scene(&mut self, camera:&engine_sdk::Camera, scene:&engine_sdk::Scene) {
        let tex = 0;
        if let Some(model) = self.models.get_mut(&tex) {
            let mut vertices = Vec::with_capacity(scene.sprites.len() * 6);
            for sprite in scene.sprites.iter() {
                if sprite.tex == tex {

                    /*
                    
                    let verts = array(
        vec2(0., 0.),
        vec2(s, s),
        vec2(0., s),
        vec2(0., 0.),
        vec2(s, 0.),
        vec2(s, s),
    );

    let index = i32(in_vertex_index) % 6;*/
                    /*for _ in 0..6 {
                        vertices.push(Vertex {
                            position: sprite.pos.into(),
                            color: [1.0, 1.0, 1.0, 1.0],
                        });
                    }*/
                    let s = 0.01;
                    vertices.push(Vertex {
                        position: (sprite.pos + Vec3::new(-s, -s, 0.0)).into(),
                        color: [1.0, 1.0, 1.0, 1.0],
                    });
                    vertices.push(Vertex {
                        position: (sprite.pos + Vec3::new(s, s, 0.0)).into(),
                        color: [1.0, 1.0, 1.0, 1.0],
                    });
                    vertices.push(Vertex {
                        position: (sprite.pos + Vec3::new(-s, s, 0.0)).into(),
                        color: [1.0, 1.0, 1.0, 1.0],
                    });
                    vertices.push(Vertex {
                        position: (sprite.pos + Vec3::new(-s, -s, 0.0)).into(),
                        color: [1.0, 1.0, 1.0, 1.0],
                    });
                    vertices.push(Vertex {
                        position: (sprite.pos + Vec3::new(s, -s, 0.0)).into(),
                        color: [1.0, 1.0, 1.0, 1.0],
                    });
                    vertices.push(Vertex {
                        position: (sprite.pos + Vec3::new(s, s, 0.0)).into(),
                        color: [1.0, 1.0, 1.0, 1.0],
                    });

                }
            }
            model.set_vertices(&self.graphics.device, &self.graphics.queue, &vertices);
            model.draw(&self.graphics);
        }

       // let (output, view, mut encoder) = self.graphics.begin();
      /*  {
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
            render_pass.set_vertex_buffer(0, self.graphics.models.get(&0).unwrap().vertex_buffer.slice(..));

           // render_pass.draw(0..(size*6), 0..1);
        }*/

       // self.graphics.queue.submit(std::iter::once(encoder.finish()));
       // output.present();
    }

    fn frame_time(&self) -> std::time::Duration {
        self.diagnostics.frame_time
    }
}