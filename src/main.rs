#![feature(fn_traits)]

mod render;
mod interface;
mod util;

extern crate gl;
extern crate glfw;
extern crate core;

use gl::types::*;
use glfw::{Action, Context, Glfw, Key, OpenGlProfileHint, SwapInterval, Window, WindowHint};
use std::ptr;
use std::time::Instant;

use rusttype::Font;
use crate::render::grid::Grid;
use crate::interface::screen::Screen;
use crate::util::vector::{Vector2, Vector2d};

pub struct Application {
    aspect_ratio: f32,
    width: u32,
    height: u32,
    window_width: u32,
    window_height: u32,
    cursor_position: Vector2d,
    grid_position: Vector2,
    delta_time: u128,
}

const VERTEX_SHADER_SOURCE: &[u8] = b"
#version 330 core
in vec3 aVertexPosition;
in vec2 aTextureCoord;
in vec4 aFgColor;
in vec4 aBgColor;
out vec2 iUv;
out vec4 iFgColor;
out vec4 iBgColor;
void main() {
    gl_Position = vec4(aVertexPosition, 1.0);
    iUv = aTextureCoord;
    iFgColor = aFgColor;
    iBgColor = aBgColor;
}\0";

const FRAGMENT_SHADER_SOURCE: &[u8] = b"
#version 330 core
in vec2 iUv;
in vec4 iFgColor;
in vec4 iBgColor;
uniform sampler2D uSampler;
uniform vec4 uFgColor;
void main() {
    vec4 textureSample = texture(uSampler, iUv);
    // gl_FragColor = textureSample;
    // gl_FragColor = vec4(iUv.x, iUv.y, 1.0, 1.0);
    gl_FragColor = mix(iBgColor, iFgColor, textureSample.x);
}\0";

const UNIFONT_DATA:&[u8] = include_bytes!("unifont-15.0.01.ttf");
pub static mut UNIFONT: Option<Font> = None;

fn main() {
    unsafe{ UNIFONT = Font::try_from_bytes(UNIFONT_DATA); };

    let mut app: Application = Application{
        aspect_ratio: 16.0/9.0,
        width: 1280,
        height: 720,
        window_width: 1280,
        window_height: 720,
        cursor_position: Vector2d::new(0.0, 0.0),
        grid_position: Vector2::new(0, 0),
        delta_time: 0,
    };

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)
        .expect("Failed to init GLFW.");
    Glfw::window_hint(&mut glfw, WindowHint::ContextVersion(3, 3));
    Glfw::window_hint(&mut glfw, WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    let (mut window, events) = glfw
        .create_window(app.width, app.height, "CONSOLE_GRID", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();
    window.set_framebuffer_size_polling(true);
    // window.set_size_limits(Some(1280), Some(720), None, None);

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

    let mut screen = Screen::new(shader_program);

    while !window.should_close() {
        let start_frame_time = Instant::now();
        app.cursor_position = get_mouse_position(&app, &window);

        let grid_pos_x = (app.cursor_position.x / app.width as f64 * screen.grid_width as f64).floor() as i32;
        let grid_pos_y = (app.cursor_position.y / app.height as f64 * screen.grid_height as f64).floor() as i32;
        app.grid_position = Vector2::new(grid_pos_x, grid_pos_y);

        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut app, &mut window, event);
        }

        screen.update(&app);

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            screen.grid.draw();
        }

        window.swap_buffers();
        glfw.poll_events();
        app.delta_time = start_frame_time.elapsed().as_millis();
    }
}

fn handle_window_event(app: &mut Application, window: &mut Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        glfw::WindowEvent::FramebufferSize(width, height) => framebuffer_resize_event(app, width as f32, height as f32),
        _ => {}
    }
}

fn get_mouse_position(app: &Application, window: &Window) -> Vector2d {
    let window_offset_x = (app.window_width - app.width)/2;
    let window_offset_y = (app.window_height - app.height)/2;
    let (mouse_pos_x, mouse_pos_y) = window.get_cursor_pos();
    let mouse_pos_x = mouse_pos_x - window_offset_x as f64;
    let mouse_pos_y = (app.height as f64 + window_offset_y as f64) - mouse_pos_y;
    Vector2d::new(mouse_pos_x, mouse_pos_y)
}

fn framebuffer_resize_event(app: &mut Application, width: f32, height:f32) {
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
    gl::BufferSubData::load_with(|_s| window.get_proc_address("glBufferSubData"));
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
    gl::GetAttribLocation::load_with(|_s| window.get_proc_address("glGetAttribLocation"));
    gl::GetBooleanv::load_with(|_s| window.get_proc_address("GetBooleanv"));
    gl::GetError::load_with(|_s| window.get_proc_address("glGetError"));
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
    gl::VertexAttribDivisor::load_with(|_s| window.get_proc_address("glVertexAttribDivisor"));
    gl::VertexAttribPointer::load_with(|_s| window.get_proc_address("glVertexAttribPointer"));

}
