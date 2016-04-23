#[macro_use]
extern crate glium;
extern crate tobj;

mod shader_helper;

use std::path::Path;

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s[0], u[0], f[0], 0.0],
        [s[1], u[1], f[1], 0.0],
        [s[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}

fn main() {
    use glium::{DisplayBuild, Surface};

    if cfg!(debug_assertions) {
        println!("Debug Build!");
    } else {
        println!("Release Build!");
    }

    let display = if cfg!(debug_assertions) {
        use glium::debug::DebugCallbackBehavior;
        glium::glutin::WindowBuilder::new()
            .with_depth_buffer(24)
            .build_glium_debug(DebugCallbackBehavior::PrintAll).unwrap()
    } else {
        use glium::glutin::Robustness;
        glium::glutin::WindowBuilder::new()
            .with_gl_robustness(Robustness::NoError)
            .with_depth_buffer(24)
            .build_glium().unwrap()
    };

    let rock = tobj::load_obj(&Path::new("pics/Tree_02.obj"));
    assert!(rock.is_ok());
    let (models, materials) = rock.unwrap();

    tobj::print_model_info(&models[..], &materials[..]);

    let mut shape = Vec::new();
    let mut normals = Vec::new();
    let mut mat_ids = Vec::new();
    let mesh = &models[0].mesh;
    for f in 0..mesh.positions.len() / 3 {
        shape.push(Vertex { position: [mesh.positions[f * 3], mesh.positions[f * 3 + 1], mesh.positions[f * 3 + 2]] });
        mat_ids.push(MatId{mat_id: 0});
    }

    for f in 0..mesh.normals.len() / 3 {
        normals.push(Normal { normal: [mesh.normals[f * 3], mesh.normals[f * 3 + 1], mesh.normals[f * 3 + 2]] });
    }

    let positions = glium::VertexBuffer::new(&display, &shape).unwrap();
    let normals = glium::VertexBuffer::new(&display, &normals).unwrap();
    let mat_ids = glium::VertexBuffer::new(&display, &mat_ids).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                          &mesh.indices).unwrap();

    let program = shader_helper::create_shader_program(&display, "shaders/basic.vert", "shaders/basic.frag", None).unwrap();

    loop {
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = std::f32::consts::PI / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f * aspect_ratio,   0.0,              0.0              , 0.0],
                [       0.0      ,    f ,              0.0              , 0.0],
                [       0.0      ,   0.0,  (zfar+znear)/(zfar-znear)    , 1.0],
                [       0.0      ,   0.0, -(2.0*zfar*znear)/(zfar-znear), 0.0],
            ]
        };

        let uniforms = uniform! {
            perspective: perspective,
            view: view_matrix(&[2.0, -1.0, 1.0], &[-2.0, 1.0, 1.0], &[0.0, 1.0, 0.0]),
            model: [
                [0.5, 0.0, 0.0, 0.0],
                [0.0, 0.5, 0.0, 0.0],
                [0.0, 0.0, 0.5, 0.0],
                [0.0, 0.0, 2.0, 1.0f32],
            ],
            diffuse0: materials[0].diffuse,
            ambient0: materials[0].ambient,
            specular0: materials[0].specular,
            u_light: [-1.0, 0.4, 0.9f32],
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        target.draw((&positions, &normals, &mat_ids), &indices, &program, &uniforms, &params).unwrap();
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
    position: [f32; 3],
}

implement_vertex!(Vertex, position);

#[derive(Copy, Clone)]
struct Normal {
    normal: [f32; 3],
}

implement_vertex!(Normal, normal);

#[derive(Copy, Clone)]
struct MatId {
    mat_id: u32,
}

implement_vertex!(MatId, mat_id);
