mod quad;

extern crate gl;
extern crate glfw;
extern crate core;

use gl::types::*;
use glfw::{Action, Context, Glfw, Key, OpenGlProfileHint, Window, WindowHint};
use std::ptr;
use quad::Quad;

const WIDTH:u32 = 800;
const HEIGHT:u32 = 600;

const VERTEX_SHADER_SOURCE: &[u8] = b"
#version 330 core
layout (location = 0) in vec3 aPos;
void main() {
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}\0";

const FRAGMENT_SHADER_SOURCE: &[u8] = b"
#version 330 core
out vec4 FragColor;

void main() {
    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}\0";

fn main() -> Result<(), ()> {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    Glfw::window_hint(&mut glfw, WindowHint::ContextVersion(3, 3));
    Glfw::window_hint(&mut glfw, WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    let (mut window, events) = glfw
        .create_window(WIDTH, HEIGHT, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();
    window.set_framebuffer_size_polling(true);

    load_gl_functions(&mut window);

    let vertex_shader: u32;
    let fragment_shader: u32;
    let shader_program: u32;

    unsafe {
        gl::Viewport(0, 0, WIDTH as i32, HEIGHT as i32);

        //Shader
        vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(vertex_shader, 1, [VERTEX_SHADER_SOURCE.as_ptr() as *const GLchar].as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);
        check_compile_status_shader(vertex_shader);

        fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(fragment_shader, 1, [FRAGMENT_SHADER_SOURCE.as_ptr() as *const GLchar].as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);
        check_compile_status_shader(fragment_shader);

        shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);
        check_link_status_program(shader_program);
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        gl::UseProgram(shader_program);
    }

    let quad = Quad::new();

    while !window.should_close() {
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            quad.draw();
        }

        window.swap_buffers();
        glfw.poll_events();
    }

    Ok(())
}

fn handle_window_event(window: &mut Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}

unsafe fn check_compile_status_shader(shader: u32) {
    let mut status: i32 = -1;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
    if status != gl::TRUE as i32 {
        let mut error_length: i32 = 0;
        let mut info_log: Vec<u8> = Vec::with_capacity(512_usize);
        gl::GetShaderInfoLog(shader, 512, &mut error_length, info_log.as_mut_ptr() as *mut _);
        info_log.set_len(error_length as usize);
        let _value = String::from_utf8_lossy(&info_log).to_string();
        panic!("{_value}");
    }
}

unsafe fn check_link_status_program(program: u32)
{
    let mut status: i32 = -1;
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
    if status != gl::TRUE as i32 {
        let mut error_length: i32 = 0;
        let mut info_log: Vec<u8> = Vec::with_capacity(512_usize);
        gl::GetProgramInfoLog(program, 512, &mut error_length, info_log.as_mut_ptr() as *mut _);
        info_log.set_len(error_length as usize);
        let _value = String::from_utf8_lossy(&info_log).to_string();
        panic!("{_value}");
    }
}

fn load_gl_functions(window: &mut Window) {
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
    gl::DrawElements::load_with(|_s| window.get_proc_address("glDrawElements"));
    gl::EnableVertexAttribArray::load_with(|_s| window.get_proc_address("glEnableVertexAttribArray"));
    gl::GenBuffers::load_with(|_s| window.get_proc_address("glGenBuffers"));
    gl::GenVertexArrays::load_with(|_s| window.get_proc_address("glGenVertexArrays"));
    gl::GetBooleanv::load_with(|_s| window.get_proc_address("GetBooleanv"));
    gl::GetProgramInfoLog::load_with(|_s| window.get_proc_address("glGetProgramInfoLog"));
    gl::GetProgramiv::load_with(|_s| window.get_proc_address("glGetProgramiv"));
    gl::GetShaderInfoLog::load_with(|_s| window.get_proc_address("glGetShaderInfoLog"));
    gl::GetShaderiv::load_with(|_s| window.get_proc_address("glGetShaderiv"));
    gl::LinkProgram::load_with(|_s| window.get_proc_address("glLinkProgram"));
    gl::ShaderSource::load_with(|_s| window.get_proc_address("glShaderSource"));
    gl::UseProgram::load_with(|_s| window.get_proc_address("glUseProgram"));
    gl::Viewport::load_with(|_s| window.get_proc_address("glViewport"));
    gl::VertexAttribPointer::load_with(|_s| window.get_proc_address("glVertexAttribPointer"));
}
