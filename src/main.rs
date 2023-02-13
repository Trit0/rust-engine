#[macro_use]
extern crate glium;

use glium::{Display, glutin, IndexBuffer, Program, Surface, VertexBuffer};
use glium::glutin::event::Event;
use glium::glutin::event_loop::{ControlFlow, EventLoopWindowTarget};
use crate::core::engine::Engine;
use crate::core::game_object::GameObject;
use crate::core::mesh::{Normal, Vertex};
use crate::core::printer::Printer;
use crate::core::scene::Scene;
use crate::core::textured_mesh::TexturedMesh;
use crate::core::timer::Timer;
use crate::core::transform::Transform;

mod math;
mod core;
mod design_pattern;
mod utils;

pub struct Context<'a> {
    pub engine: &'a Engine<'a>,
    pub display: &'a Display,
    pub program: &'a Program,
    pub positions: VertexBuffer<Vertex>,
    pub normals: VertexBuffer<Normal>,
    pub indices: IndexBuffer<u16>
}

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let s = b"Hello, world!";
    let ss = &s[7..12];
    println!("{:?}", ss);

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_title("Rust Engine");
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let printer = Box::new(Printer { name: String::from("Joe") });
    let printer2 = Box::new(Printer { name: String::from("Donald") });
    let printer3 = Box::new(Printer { name: String::from("Elon")});
    let printer4 = Box::new(Printer { name: String::from("Suzy") });
    let printer5 = Box::new(Printer { name: String::from("Jacob") });
    let printer6 = Box::new(Printer { name: String::from("Mohammed")});

    let mesh = TexturedMesh::load("assets/crate.obj");
    let positions = glium::VertexBuffer::new(&display, &mesh.mesh.vertices).unwrap();
    let normals = glium::VertexBuffer::new(&display, &mesh.mesh.normals).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &mesh.mesh.indices).unwrap();

    let go = Box::new( GameObject { components: vec![printer, printer2, printer3], transform: Transform::empty() });
    let go2 = Box::new( GameObject { components: vec![printer4, printer5, printer6], transform: Transform::empty() });

    let scene = Scene { game_objects: vec![go, go2] };

    let mut engine = Engine { scenes: vec![scene], timer: Timer::new() };

    let vertex_shader_src = r#"
        #version 140
        in vec3 position;
        in vec3 normal;
        uniform mat4 matrix;
        void main() {
            gl_Position = matrix * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src,
                                              None).unwrap();

    // let mut context = Context {
    //     engine: &engine,
    //     display: &display,
    //     program: &program,
    //     positions,
    //     normals,
    //     indices
    // };

    event_loop.run(move |event, target, control_flow| event_handler(event, target, control_flow, &mut engine, &display, &program, &positions, &normals, &indices));
}

fn event_handler(event: Event<()>, target: &EventLoopWindowTarget<()>, control_flow: &mut ControlFlow, engine: &mut Engine, display: &Display, program: &Program, positions: &VertexBuffer<Vertex>, normals: &VertexBuffer<Normal>, indices: &IndexBuffer<u16>) {
    let next_frame_time = std::time::Instant::now() +
        std::time::Duration::from_nanos(16_666_667);
    *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

    let scene = engine.scenes.first().unwrap();

    let ellapsed_time= engine.timer.set_time_and_get_ellapsed(std::time::Instant::now());
    //let ellapsed_time = 0.0;
    scene.update(ellapsed_time);
    scene.late_update();

    match event {
        glutin::event::Event::WindowEvent { event, .. } => match event {
            glutin::event::WindowEvent::CloseRequested => {
                scene.close();
                *control_flow = glutin::event_loop::ControlFlow::Exit;
                scene.dispose();
                return;
            },
            _ => return,
        },
        glutin::event::Event::NewEvents(cause) => match cause {
            glutin::event::StartCause::ResumeTimeReached { .. } => (),
            glutin::event::StartCause::Init => {
                scene.initialize();
                scene.start();
            },
            _ => return,
        },
        _ => return,
    }

    let mut target = display.draw();
    target.clear_color_and_depth((0.2, 0.7, 1.0, 1.0), 1.0);

    let matrix = [
        [0.5, 0.0, 0.0, 0.0],
        [0.0, 0.5, 0.0, 0.0],
        [0.0, 0.0, 0.5, 0.0],
        [0.0, 0.0, 0.0, 1.0f32]
    ];

    target.draw((positions, normals), indices, program, &uniform! { matrix: matrix },
                &Default::default()).unwrap();

    target.finish().unwrap();
}
