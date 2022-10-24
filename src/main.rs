mod quad;
mod glyph_info;

extern crate gl;
extern crate glfw;
extern crate core;

use std::collections::HashMap;
use gl::types::*;
use glfw::{Action, Context, Glfw, Key, OpenGlProfileHint, Window, WindowHint};
use std::ptr;
use std::time::Instant;
use quad::Quad;
use rand::Rng;
use rusttype::{Font};
use crate::glyph_info::{FONT, GLYPH_CACHE};

const WIDTH:u32 = 1280;
const HEIGHT:u32 = 720;

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
    // FragColor = uColor;
    vec4 textureSample = texture(uSampler, iUv);
    if (textureSample.r > 0.0f) FragColor = uFgColor * textureSample;
    else FragColor = uBgColor;
}\0";

const FONT_DATA:&[u8] = include_bytes!("unifont-15.0.01.ttf");

fn main() -> Result<(), ()> {
    unsafe{
        GLYPH_CACHE = Some(HashMap::new());
        FONT = Font::try_from_bytes(FONT_DATA);
    };

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

    let vertex_shader: GLuint;
    let fragment_shader: GLuint;
    let shader_program: GLuint;

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
    }

    let mut rng = rand::thread_rng();
    let mut quads: Vec<Quad> = vec![];

    let width = 16 * 5;
    let height = 9 * 5;
    let width_f = width as f32;
    let height_f = height as f32;

    let text: Vec<char> = "Tema la taille de la glyphe.".chars().collect();
    let len_text = text.len();

    for x in 0..width {
        for y in 0..height {
            let start_x = ((x as f32)       / width_f ) * 2.0 - 1.0;
            let end_x =   ((x as f32 + 1.0) / width_f ) * 2.0 - 1.0;
            let start_y = ((y as f32)       / height_f) * 2.0 - 1.0;
            let end_y =   ((y as f32 + 1.0) / height_f) * 2.0 - 1.0;
            let new_char = text[(x + width * y) % len_text];
            // let new_char = char::from_u32(x + width * y).unwrap_or('�');
            let fg_color = [rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>(), 1.0];
            let bg_color = [0.0, 0.0, 0.0, 1.0];
            let quad = Quad::new([start_x, end_x, start_y, end_y], fg_color, bg_color, shader_program, new_char);
            quads.push(quad);
        }
    }

    while !window.should_close() {
        let now = Instant::now();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        // for quad in quads.as_mut_slice() {
        //     let char = char::from_u32((rng.gen::<f32>() * 65000.0) as u32).unwrap_or('a');
        //     quad.switch_char(char);
        // }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            for quad in quads.as_slice() {
                quad.draw();
            }
        }

        window.swap_buffers();
        glfw.poll_events();
        let time_elapsed = now.elapsed().as_millis();
        println!("{time_elapsed}");
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
    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
    let mut width_c = width;
    let mut height_c = height;
    if width < height {
        height_c = 1.0/aspect_ratio * width_c;
    } else {
        width_c = aspect_ratio * height_c;
        if width_c > width {
            width_c = width;
            height_c = 1.0/aspect_ratio * width_c;
        }
    }
    let offset_w = (width - width_c) / 2.0;
    let offset_h = (height - height_c) / 2.0;
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
