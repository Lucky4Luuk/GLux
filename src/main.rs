//File for testing
extern crate glux;
use glux::{
    Program, WindowSettings,
    shader::{Shader, ShaderProgram},
    mesh::{Vertex, Mesh},
};

fn main() {
    let win_settings = WindowSettings {
        title: "test",
        resolution: (1280, 720),
        gl_version: (4, 5),
        vsync: false,
    };

    let mut program = Program::new(win_settings);

    let vert_shader = Shader::from_source(include_str!("vs.glsl"), gl::VERTEX_SHADER).unwrap();
    let frag_shader = Shader::from_source(include_str!("fs.glsl"), gl::FRAGMENT_SHADER).unwrap();
    let shader_program = ShaderProgram::from_shaders(vec![&vert_shader, &frag_shader]);
    let vertices: Vec<Vertex> = vec![
            Vertex {
                pos: (-0.5, -0.5, 0.0).into(),
                uv: (0.0, 0.0).into(),
                rgba: (1.0, 0.0, 0.0, 1.0).into(),
            },
            Vertex {
                pos: (0.5, -0.5, 0.0).into(),
                uv: (1.0, 0.0).into(),
                rgba: (0.0, 1.0, 0.0, 1.0).into(),
            },
            Vertex {
                pos: (0.5, 0.5, 0.0).into(),
                uv: (1.0, 1.0).into(),
                rgba: (0.0, 0.0, 1.0, 1.0).into(),
            },

            Vertex {
                pos: (-0.5, 0.5, 0.0).into(),
                uv: (0.0, 1.0).into(),
                rgba: (0.0, 1.0, 0.0, 1.0).into(),
            },
            Vertex {
                pos: (-0.5, -0.5, 0.0).into(),
                uv: (0.0, 0.0).into(),
                rgba: (1.0, 0.0, 0.0, 1.0).into(),
            },
            Vertex {
                pos: (0.5, 0.5, 0.0).into(),
                uv: (1.0, 1.0).into(),
                rgba: (0.0, 0.0, 1.0, 1.0).into(),
            },
        ];
    let quad = Mesh::from_vertices(&vertices);

    let mut event_pump = program.sdl_mut().event_pump().unwrap();
    'program: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'program,
                _ => {},
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        shader_program.bind();
        quad.draw();
        shader_program.unbind();

        program.sdl_window().gl_swap_window();
    }
}
