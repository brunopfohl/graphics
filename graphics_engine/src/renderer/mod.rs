mod shader;

use gl::types::*;
use glutin::event_loop::EventLoop;
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use crate::graphics::Sprite;
use self::shader::Shader;

pub struct Renderer {
    vao: GLuint,
    vbo: GLuint,
    shader: Shader,
}

impl Renderer {
    pub fn new() -> (Renderer, glutin::WindowedContext<glutin::PossiblyCurrent>, EventLoop<()>) {
        let el = EventLoop::new();
        let wb = WindowBuilder::new()
            .with_title("Graphics Engine")
            .with_inner_size(glutin::dpi::LogicalSize::new(800.0, 600.0));
        
        let windowed_context = ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(wb, &el)
            .unwrap();
        
        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        gl::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _);

        let vertex_shader = r#"
            #version 330 core
            layout (location = 0) in vec2 aPos;
            void main() {
                gl_Position = vec4(aPos.x, aPos.y, 0.0, 1.0);
            }
        "#;

        let fragment_shader = r#"
            #version 330 core
            out vec4 FragColor;
            void main() {
                FragColor = vec4(1.0, 0.5, 0.2, 1.0);  // Orange color
            }
        "#;

        let shader = Shader::new(vertex_shader, fragment_shader)
            .expect("Failed to create shader program");

        let mut vao = 0;
        let mut vbo = 0;
        
        unsafe {
            // Set viewport size
            let size = windowed_context.window().inner_size();
            gl::Viewport(0, 0, size.width as i32, size.height as i32);

            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            
            // Fix stride and offset calculation
            gl::VertexAttribPointer(
                0,  // attribute index 
                2,  // size (vec2)
                gl::FLOAT,  // type
                gl::FALSE,  // normalized
                2_i32 * std::mem::size_of::<f32>() as i32,  // stride
                0 as *const _  // offset
            );
            gl::EnableVertexAttribArray(0);

            // For 2D rendering, we don't need depth testing
            gl::Disable(gl::DEPTH_TEST);
        }

        (Renderer { 
            vao, 
            vbo, 
            shader
        }, windowed_context, el)
    }

    pub fn draw_sprite(&self, sprite: &Sprite) {
        let vertices: Vec<f32> = sprite.get_transformed_vertices()
            .iter()
            .flat_map(|v| vec![v.x, v.y])
            .collect();

        unsafe {
            // Clear both color and depth buffers
            gl::Clear(gl::COLOR_BUFFER_BIT);
            
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
                vertices.as_ptr() as *const _,
                gl::DYNAMIC_DRAW,
            );

            self.shader.use_program();
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, vertices.len() as i32 / 2);
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteProgram(self.shader.id);
        }
    }
}
