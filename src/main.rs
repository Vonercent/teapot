#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rodio::{OutputStream, Source};
use std::io::Cursor;
use glium::{uniform, Surface};

mod teapot;

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

    let pitch: f32 = 0.0;
    let roll: f32 = 0.0;

    // Pitch and roll is never mutated, to save CPU usage, precompute the sin and cos here
    let pitch_sin = pitch.sin();
    let pitch_cos = pitch.cos();

    let roll_sin = roll.sin();
    let roll_cos = roll.cos();

    let mut yaw: f32 = 0.0; // Roll is mutated, compute it every frame

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Create a source from the audio bytes
    let source = rodio::Decoder::new(Cursor::new(include_bytes!("../funkytown.mp3"))).unwrap();

    let _ = stream_handle.play_raw(source.repeat_infinite().convert_samples());

    let mut before = std::time::Instant::now();

    let _ = event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                glium::winit::event::WindowEvent::RedrawRequested => {
                    let delta = std::time::Instant::now() - before;

                    let delta_ms = delta.as_micros() as f32 / 1000.0; // More accuracy

                    before = std::time::Instant::now();

                    let mut target = display.draw();
                    // Clear screen
                    target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

                    yaw += delta_ms/150.0;
                    
                    let yaw_sin = yaw.sin();
                    let yaw_cos = yaw.cos();
                    
                    let pitch_matrix = [
                        [1.0, 0.0, 0.0, 0.0],
                        [0.0, pitch_cos, -pitch_sin, 0.0],
                        [0.0, pitch_sin, pitch_cos, 0.0],
                        [0.0, 0.0, 0.0, 1.0],
                    ];

                    let roll_matrix = [
                        [roll_cos, -roll_sin, 0.0, 0.0],
                        [roll_sin, roll_cos, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0],
                    ];
                    
                    let yaw_matrix = [
                        [yaw_cos, 0.0, yaw_sin, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [-yaw_sin, 0.0, yaw_cos, 0.0],
                        [0.0, 0.0, 0.0, 1.0]
                    ];

                    let scale_matrix = [
                        [0.01, 0.0, 0.0, 0.0],  // x
                        [0.0, 0.01, 0.0, 0.0],  // y
                        [0.0, 0.0, 0.01, 0.0],  // z 
                        [0.0, 0.0, 0.0, 1.0f32] // w
                    ];
                    

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
                        pitch_matrix: pitch_matrix,
                        roll_matrix: roll_matrix,
                        yaw_matrix: yaw_matrix,
                        scale_matrix: scale_matrix,
                        u_light: [-1.0, 0.4, 0.9f32]
                    },
                        &params);
                
                    // Draw to screen
                    target.finish().unwrap();
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
