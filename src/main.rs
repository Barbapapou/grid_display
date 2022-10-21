extern crate gl;
extern crate glfw;

use glfw::{Action, Context, Glfw, Key, OpenGlProfileHint, WindowHint};
use std::ffi::c_void;

fn main() -> Result<(), ()> {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    Glfw::window_hint(&mut glfw, WindowHint::ContextVersion(4, 6));
    Glfw::window_hint(&mut glfw, WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    let (mut window, events) = glfw
        .create_window(800, 600, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();
    window.set_framebuffer_size_polling(true);

    load_gl_functions(&mut window);

    let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

    let mut vbo: u32 = 0;
    let mut vao: u32 = 0;
    let vertex_shader: u32;
    let fragment_shader: u32;
    let shader_program: u32;

    let vertex_shader_source = include_str!("vs.glsl").as_ptr() as *const *const gl::types::GLchar;
    let fragment_shader_source = include_str!("fs.glsl").as_ptr() as *const *const gl::types::GLchar;

    unsafe {
        gl::Viewport(0, 0, 800, 600);

        //Shader
        vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(vertex_shader, 1, vertex_shader_source, &0);
        gl::CompileShader(vertex_shader);

        fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(fragment_shader, 1, fragment_shader_source, &0);
        gl::CompileShader(fragment_shader);

        shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        //Vertex info
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, 9 * 4, vertices.as_ptr() as *const c_void, gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * 4, std::ptr::null::<c_void>());
        gl::EnableVertexAttribArray(0);

        //Pre draw
        gl::UseProgram(shader_program);
        gl::BindVertexArray(vao);
    }

    while !window.should_close() {
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.swap_buffers();
        glfw.poll_events();
    }

    Ok(())
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}

fn load_gl_functions(window: &mut glfw::Window) {
    gl::AttachShader::load_with(|_s| window.get_proc_address("glAttachShader"));
    gl::BindBuffer::load_with(|_s| window.get_proc_address("glBindBuffer"));
    gl::BindVertexArray::load_with(|_s| window.get_proc_address("glBindVertexArray"));
    gl::BufferData::load_with(|_s| window.get_proc_address("glBufferData"));
    gl::Clear::load_with(|_s| window.get_proc_address("glClear"));
    gl::ClearColor::load_with(|_s| window.get_proc_address("glClearColor"));
    gl::CompileShader::load_with(|_s| window.get_proc_address("glCompileShader"));
    gl::CreateProgram::load_with(|_s| window.get_proc_address("glCreateProgram"));
    gl::CreateShader::load_with(|_s| window.get_proc_address("glCreateShader"));
    gl::DeleteShader::load_with(|_s| window.get_proc_address("glDeleteShader"));
    gl::DrawArrays::load_with(|_s| window.get_proc_address("glDrawArrays"));
    gl::EnableVertexAttribArray::load_with(|_s| window.get_proc_address("glEnableVertexAttribArray"));
    gl::GenBuffers::load_with(|_s| window.get_proc_address("glGenBuffers"));
    gl::GenVertexArrays::load_with(|_s| window.get_proc_address("glGenVertexArrays"));
    gl::LinkProgram::load_with(|_s| window.get_proc_address("glLinkProgram"));
    gl::ShaderSource::load_with(|_s| window.get_proc_address("glShaderSource"));
    gl::UseProgram::load_with(|_s| window.get_proc_address("glUseProgram"));
    gl::Viewport::load_with(|_s| window.get_proc_address("glViewport"));
    gl::VertexAttribPointer::load_with(|_s| window.get_proc_address("glVertexAttribPointer"));
}
