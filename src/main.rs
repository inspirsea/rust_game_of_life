extern crate gl;
extern crate sdl2;

mod game_of_life;
mod render_gl;

use game_of_life::Game;
use std::ffi::CString;
use std::time::{Instant};

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let mut initial_values: Vec<(u32, u32)> = vec![];

    if let Ok(lines) = read_lines("./rle2.life") {
        // Consumes the iterator, returns an (Optional) String
        for (i, line) in lines.enumerate() {
            if let Ok(ip) = line {

                for (j, c) in ip.chars().enumerate() {
                    if c == '*' {
                        initial_values.push((i as u32, j as u32));
                    }
                    // do something with character `c` and index `i`
                }

                println!("{}", ip);
            }
        }
    }

    let mut game = Game::init(500, initial_values);

    let sdl = sdl2::init().unwrap();

    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window("Game", 1000, 1000)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        
    unsafe {
        let window_size = window.size();
        gl::Viewport(0, 0, window_size.0 as i32, window_size.1 as i32);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let vert_shader =
        render_gl::Shader::from_vert_source(&CString::new(include_str!("vert.shader")).unwrap())
            .unwrap();

    let frag_shader =
        render_gl::Shader::from_frag_source(&CString::new(include_str!("frag.shader")).unwrap())
            .unwrap();

    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();
    shader_program.set_used();

    let mut vertices: Vec<f32> = vec![];

    game.render(&mut vertices);

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,                                                       // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW,                               // usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
    }

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }

    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            0,         // index of the generic vertex attribute ("layout (location = 0)")
            3,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null(),                                     // offset of the first component
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    let fps = 20;
    let fps_delta = 1000/fps;
    let mut last_frame_time = Instant::now();

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        unsafe {
            let window_size = window.size();
            gl::Viewport(0, 0, window_size.0 as i32, window_size.1 as i32);
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        }
        
        let frame_time = Instant::now().duration_since(last_frame_time).as_millis();

        if frame_time > fps_delta {

            let timer = Instant::now();

            game.update();

            game.render(&mut vertices);

            unsafe {
                gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
                gl::BufferData(
                    gl::ARRAY_BUFFER,                                                       // target
                    (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
                    vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
                    gl::STATIC_DRAW,                               // usage
                );
                gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
            }

            unsafe {
                gl::BindVertexArray(vao);
                gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        
                gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
                gl::VertexAttribPointer(
                    0,         // index of the generic vertex attribute ("layout (location = 0)")
                    3,         // the number of components per generic vertex attribute
                    gl::FLOAT, // data type
                    gl::FALSE, // normalized (int-to-float conversion)
                    (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                    std::ptr::null(),                                     // offset of the first component
                );
        
                gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                gl::BindVertexArray(0);
            }

            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
    
            shader_program.set_used();
    
            unsafe {
                gl::BindVertexArray(vao);
                gl::DrawArrays(
                    gl::TRIANGLES, // mode
                    0,             // starting index in the enabled arrays
                    vertices.len() as i32,             // number of indices to be rendered
                );
            }
    
            window.gl_swap_window();

            last_frame_time = Instant::now();

            println!("{}", Instant::now().duration_since(timer).as_micros());
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}