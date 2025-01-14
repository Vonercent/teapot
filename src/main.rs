#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rodio::{OutputStream, Source};
use std::io::Cursor;
use glium::{uniform, Surface};

mod teapot;
mod matrices;

const VERTEX_SHADER_SRC: &str = include_str!("../shaders/vertex_shader.vert");
const FRAGMENT_SHADER_SRC: &str = include_str!("../shaders/fragment_shader.frag");


fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder().build().expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
    .with_title("TEAPOT")
    .build(&event_loop);

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();

    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &teapot::INDICES).unwrap();

    let program = glium::Program::from_source(&display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Create a source from the audio bytes
    let source = rodio::Decoder::new(Cursor::new(include_bytes!("../funkytown.mp3"))).unwrap();

    let _ = stream_handle.play_raw(source.repeat_infinite().convert_samples());
    
    let start = std::time::Instant::now(); // Used to calculate time elapsed since program started

    let _ = event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                glium::winit::event::WindowEvent::RedrawRequested => {
                    let mut target = display.draw();
                    // Clear screen
                    target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

                    let yaw = (std::time::Instant::now()-start).as_secs_f32()*6.5;
                    
                    let yaw_sin = yaw.sin();
                    let yaw_cos = yaw.cos();
                    
                    let yaw_matrix = [
                        [yaw_cos, 0.0, yaw_sin, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [-yaw_sin, 0.0, yaw_cos, 0.0],
                        [0.0, 0.0, 0.0, 1.0]
                    ];

                    let model = matrices::move_and_scale(0.0, 0.0, 0.0, 0.01);
                    let view = matrices::view_matrix(&[0.0, 0.05, -2.0], &[-0.03, 0.0, 1.0], &[0.0, 1.0, 0.0]);
                    let perspective = matrices::perspective(&target);

                    let params = glium::DrawParameters {
                        depth: glium::Depth {
                            test: glium::DepthTest::IfLess,
                            write: true,
                            .. Default::default()
                        },
                        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
                        .. Default::default()
                    };

                    let _ = target.draw((&positions, &normals), &indices, &program, &uniform! {
                        yaw_matrix: yaw_matrix,
                        model: model,
                        perspective: perspective,
                        view: view,
                        u_light: [-1.0, 0.4, 0.9f32]
                    },
                        &params);
                
                    // Draw to screen
                    target.finish().unwrap();
                },
                // Because glium doesn't know about windows we need to resize the display
                // when the window's size has changed.
                glium::winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                },
                _ => (),
            },    
            glium::winit::event::Event::AboutToWait => {
                window.request_redraw();
            },        
            _ => (),
        };
    });

}
