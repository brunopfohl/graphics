mod math;
mod graphics;
mod renderer;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use math::Vec2;
use graphics::Sprite;
use renderer::Renderer;

fn main() {
    let (renderer, context, event_loop) = Renderer::new();
    // Make the sprite smaller so it's visible in normalized coordinates
    let mut sprite = Sprite::new(0.2, 0.2);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit
            },
            Event::RedrawRequested(_) => {
                unsafe {
                    gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }

                sprite.transform.rotate(1.0); // Use 2D rotation

                renderer.draw_sprite(&sprite);

                context.swap_buffers().unwrap();
            },
            Event::MainEventsCleared => {
                context.window().request_redraw();
            },
            _ => ()
        }
    });
}
