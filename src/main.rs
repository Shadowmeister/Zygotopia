#[macro_use]
extern crate glium;

fn main() {
    use glium::{DisplayBuild, Surface};

    if cfg!(debug_assertions) {
        println!("Debug Build!");
    } else {
        println!("Release Build!");
    }

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;


    let display = if cfg!(debug_assertions) {
        use glium::debug::DebugCallbackBehavior;
        glium::glutin::WindowBuilder::new().build_glium_debug(DebugCallbackBehavior::DebugMessageOnError).unwrap()
    } else {
        use glium::glutin::Robustness;
        glium::glutin::WindowBuilder::new()
            .with_gl_robustness(Robustness::NoError)
            .build_glium().unwrap()
    };

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();
        // listing the events produced by the window and waiting to be received
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return, // the window has been closed by the user
                _ => ()
            }
        }
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);
