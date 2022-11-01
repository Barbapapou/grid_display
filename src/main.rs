mod quad;
mod glyph_info;
mod grid;
mod box_drawing;

extern crate gl;
extern crate glfw;
extern crate core;

use std::collections::HashMap;
use gl::types::*;
use glfw::{Action, Context, Glfw, Key, OpenGlProfileHint, SwapInterval, Window, WindowHint};
use std::ptr;
use std::time::Instant;
use quad::Quad;
use rusttype::{Font};
use crate::glyph_info::{UNIFONT, GLYPH_CACHE};
use crate::grid::{Grid};

struct Application {
    aspect_ratio: f32,
    width: u32,
    height: u32,
    window_width: u32,
    window_height: u32,
}

static mut APPLICATION: Application = Application{
    aspect_ratio: 16.0/9.0,
    width: 1280,
    height: 720,
    window_width: 1280,
    window_height: 720,
};

const VERTEX_SHADER_SOURCE: &[u8] = b"
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aUv;
out vec2 iUv;
void main() {
    gl_Position = vec4(aPos, 1.0);
    iUv = aUv;
}\0";

const FRAGMENT_SHADER_SOURCE: &[u8] = b"
#version 330 core
in vec2 iUv;
out vec4 FragColor;
uniform sampler2D uSampler;
uniform vec4 uFgColor;
uniform vec4 uBgColor;
void main() {
    vec4 textureSample = texture(uSampler, iUv);
    FragColor = mix(uBgColor, uFgColor, textureSample.r);
}\0";

const UNIFONT_DATA:&[u8] = include_bytes!("unifont-15.0.01.ttf");

fn main() -> Result<(), ()> {
    unsafe{
        GLYPH_CACHE = Some(HashMap::new());
        UNIFONT = Font::try_from_bytes(UNIFONT_DATA);
    };

    let app = unsafe {&APPLICATION};

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    Glfw::window_hint(&mut glfw, WindowHint::ContextVersion(3, 3));
    Glfw::window_hint(&mut glfw, WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    let (mut window, events) = glfw
        .create_window(app.width, app.height, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();
    window.set_framebuffer_size_polling(true);
    window.set_size_limits(Some(1280), Some(720), None, None);

    load_gl_functions(&mut window);
    glfw.set_swap_interval(SwapInterval::None);

    let vertex_shader: GLuint;
    let fragment_shader: GLuint;
    let shader_program: GLuint;

    unsafe {
        gl::Viewport(0, 0, app.width as i32, app.height as i32);

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
    }

    let mul = 5;
    let grid_width = 16 * 2 * mul;
    let grid_height = 9 * mul;

    let mut grid = Grid::new(grid_width, grid_height, shader_program);
    let mut time_last_frame = String::new();
    // grid.shuffle_glyph();

    while !window.should_close() {
        let now = Instant::now();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        grid.clear();
        grid.write_at(1, 1, &time_last_frame);
        grid.write_box(0, 0, time_last_frame.len() as i32 + 1, 2);
        grid.write_at(5, 5, "Hello world!");
        let window_offset_x = (app.window_width - app.width)/2;
        let window_offset_y = (app.window_height - app.height)/2;
        let (mouse_pos_x, mouse_pos_y) = window.get_cursor_pos();
        let mouse_pos_x = mouse_pos_x - window_offset_x as f64;
        let mouse_pos_y = (app.height as f64 + window_offset_y as f64)- mouse_pos_y;
        let mouse_pos_str = format!("Mouse coordinate: {mouse_pos_x}, {mouse_pos_y}");
        grid.write_at(0,3, &mouse_pos_str);
        let grid_pos_x = (mouse_pos_x / app.width as f64 * grid_width as f64).floor() as i32;
        let grid_pos_y = (mouse_pos_y / app.height as f64 * grid_height as f64).floor() as i32;
        let mouse_pos_str = format!("Grid coordinate: {grid_pos_x}, {grid_pos_y}");
        grid.write_at(0,4, &mouse_pos_str);
        if grid_pos_x >= 0 && grid_pos_x < grid_width && grid_pos_y >= 0 && grid_pos_y < grid_height {
            grid.inverse_color_at(grid_pos_x as i32, grid_pos_y as i32);
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            grid.draw();
        }

        window.swap_buffers();
        glfw.poll_events();

        let time_elapsed = now.elapsed().as_millis();
        time_last_frame = format!("{time_elapsed} ms");
    }

    Ok(())
}

fn handle_window_event(window: &mut Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        glfw::WindowEvent::FramebufferSize(width, height) => framebuffer_resize_event(width as f32, height as f32),
        _ => {}
    }
}

fn framebuffer_resize_event(width: f32, height:f32) {
    let mut app = unsafe {&mut APPLICATION};
    app.window_width = width as u32;
    app.window_height = height as u32;
    let mut width_c = width;
    let mut height_c = height;
    if width < height {
        height_c = 1.0/app.aspect_ratio * width_c;
    } else {
        width_c = app.aspect_ratio * height_c;
        if width_c > width {
            width_c = width;
            height_c = 1.0/app.aspect_ratio * width_c;
        }
    }
    let offset_w = (width - width_c) / 2.0;
    let offset_h = (height - height_c) / 2.0;
    app.width = width_c as u32;
    app.height = height_c as u32;
    unsafe { gl::Viewport(offset_w as i32, offset_h as i32, width_c as i32, height_c as i32); }
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
    gl::ActiveTexture::load_with(|_s| window.get_proc_address("glActiveTexture"));
    gl::AttachShader::load_with(|_s| window.get_proc_address("glAttachShader"));
    gl::BindBuffer::load_with(|_s| window.get_proc_address("glBindBuffer"));
    gl::BindTexture::load_with(|_s| window.get_proc_address("glBindTexture"));
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
    gl::GenerateMipmap::load_with(|_s| window.get_proc_address("glGenerateMipmap"));
    gl::GenTextures::load_with(|_s| window.get_proc_address("glGenTextures"));
    gl::GenVertexArrays::load_with(|_s| window.get_proc_address("glGenVertexArrays"));
    gl::GetBooleanv::load_with(|_s| window.get_proc_address("GetBooleanv"));
    gl::GetProgramInfoLog::load_with(|_s| window.get_proc_address("glGetProgramInfoLog"));
    gl::GetProgramiv::load_with(|_s| window.get_proc_address("glGetProgramiv"));
    gl::GetShaderInfoLog::load_with(|_s| window.get_proc_address("glGetShaderInfoLog"));
    gl::GetShaderiv::load_with(|_s| window.get_proc_address("glGetShaderiv"));
    gl::GetUniformLocation::load_with(|_s| window.get_proc_address("glGetUniformLocation"));
    gl::LinkProgram::load_with(|_s| window.get_proc_address("glLinkProgram"));
    gl::ShaderSource::load_with(|_s| window.get_proc_address("glShaderSource"));
    gl::TexImage2D::load_with(|_s| window.get_proc_address("glTexImage2D"));
    gl::TexParameteri::load_with(|_s| window.get_proc_address("glTexParameteri"));
    gl::Uniform4f::load_with(|_s| window.get_proc_address("glUniform4f"));
    gl::UseProgram::load_with(|_s| window.get_proc_address("glUseProgram"));
    gl::Viewport::load_with(|_s| window.get_proc_address("glViewport"));
    gl::VertexAttribPointer::load_with(|_s| window.get_proc_address("glVertexAttribPointer"));
}
