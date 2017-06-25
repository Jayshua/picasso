use gl;
use gl::types::*;

use std::mem;
use std::ptr;
use std::str;
use std::ffi::CString;
use point::{Point, Color};
use figure::{Figure, FillType};


// Shader sources
static VS_SRC: &'static str = r#"
    #version 150

    in vec2 position;
    out vec2 location;

    void main() {
       gl_Position = vec4(position, 0.0, 1.0);
       location = position;
    }
"#;

static FS_SRC: &'static str = r#"
    #version 150

    in vec2 location;
    out vec4 out_color;

    uniform uint fill_type;

    uniform vec4 color;

    uniform vec4 gradient;
    uniform vec4 color_begin;
    uniform vec4 color_end;


    void main() {
        if (fill_type == uint(0)) {
            out_color = color;
        } else if (fill_type == uint(1)) {
            vec2 begin = gradient.xy;
            vec2 end = gradient.zw;

            vec2 b = end - begin;
            float multiplier = dot(location - begin, normalize(b)) / length(b);

            out_color = vec4(
                color_begin.r + multiplier * (color_end.r - color_begin.r),
                color_begin.g + multiplier * (color_end.g - color_begin.g),
                color_begin.b + multiplier * (color_end.b - color_begin.b),
                color_begin.a + multiplier * (color_end.a - color_begin.a)
            );
        }
    }
"#;

pub struct ShaderProgram {
    vao: GLuint,
    vbo: GLuint,
    program: GLuint,

    fill_type_location: GLint,

    color_location: GLint,

    gradient_location: GLint,
    color_begin_location: GLint,
    color_end_location: GLint,
}

impl ShaderProgram {
    pub fn new() -> ShaderProgram {
        let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
        let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
        let program = link_program(vs, fs);

        unsafe {
            gl::DeleteShader(fs);
            gl::DeleteShader(vs);
        }

        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            // Create Vertex Array Object
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            // Create a Vertex Buffer Object and copy the vertex data to it
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (100 * mem::size_of::<Point>()) as GLsizeiptr,
                ptr::null(),
                gl::STATIC_DRAW
            );

            // Use shader program
            gl::UseProgram(program);

            // Specify the layout of the vertex data
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());

            // Get the location of the uniform attributes
            let fill_type_str = CString::new("fill_type".as_bytes()).unwrap();
            let color_str = CString::new("color".as_bytes()).unwrap();
            let gradient_str = CString::new("gradient".as_bytes()).unwrap();
            let color_begin_str = CString::new("color_begin".as_bytes()).unwrap();
            let color_end_str = CString::new("color_end".as_bytes()).unwrap();

            ShaderProgram {
                vao: vao,
                vbo: vbo,
                program: program,

                fill_type_location: gl::GetUniformLocation(program, fill_type_str.as_ptr()),

                color_location: gl::GetUniformLocation(program, color_str.as_ptr()),

                gradient_location: gl::GetUniformLocation(program, gradient_str.as_ptr()),
                color_begin_location: gl::GetUniformLocation(program, color_begin_str.as_ptr()),
                color_end_location: gl::GetUniformLocation(program, color_end_str.as_ptr())
            }
        }
    }

    pub fn draw_figure(&self, figure: &Figure) {
        unsafe {
            // Upload the fill data
            match figure.fill {
                FillType::Solid(color) => {
                    gl::Uniform1ui(self.fill_type_location, 0);
                    gl::Uniform4f(self.color_location, color.red, color.green, color.blue, color.alpha);
                },

                FillType::LinearGradient(from, to, from_color, to_color) => {
                    gl::Uniform1ui(self.fill_type_location, 1);
                    gl::Uniform4f(self.gradient_location, from.x, from.y, to.x, to.y);
                    gl::Uniform4f(self.color_begin_location, from_color.red, from_color.green, from_color.blue, from_color.alpha);
                    gl::Uniform4f(self.color_end_location, to_color.red, to_color.green, to_color.blue, to_color.alpha);
                }
            }


            // Upload the figure to the GPU
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (mem::size_of::<Point>() * figure.points.len()) as isize,
                mem::transmute(&figure.points[0])
            );


            // Draw to the stencil buffer
            gl::Enable(gl::STENCIL_TEST);
            gl::StencilMask(0xff);
            gl::StencilFunc(gl::ALWAYS, 0, 0xff);
            gl::StencilOp(gl::INVERT, gl::INVERT, gl::INVERT);
            gl::ColorMask(gl::FALSE, gl::FALSE, gl::FALSE, gl::FALSE);

            // Draw to the stencil buffer
            for path in figure.paths.windows(2) {
                gl::DrawArrays(gl::TRIANGLE_FAN, path[0] as i32, (path[1] - path[0]) as i32);
            }

            // Draw final path
            if let Some(last) = figure.paths.last() {
                gl::DrawArrays(gl::TRIANGLE_FAN, *last as i32, (figure.points.len() - last) as i32);
            }


            // Draw to the color buffer
            gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);
            gl::StencilFunc(gl::EQUAL, 0xFF, 0xff);
            gl::StencilOp(gl::KEEP, gl::KEEP, gl::KEEP);


            for path in figure.paths.windows(2) {
                gl::DrawArrays(gl::TRIANGLE_FAN, path[0] as i32, (path[1] - path[0]) as i32);
            }

            if let Some(last) = figure.paths.last() {
                gl::DrawArrays(gl::TRIANGLE_FAN, *last as i32, (figure.points.len() - last) as i32);
            }

            gl::Disable(gl::STENCIL_TEST);
        }
    }

    pub fn drop(&self) {
        unsafe {
            gl::DeleteProgram(self.program);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}






fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        // Attempt to compile the shader
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(shader,
                                 len,
                                 ptr::null_mut(),
                                 buf.as_mut_ptr() as *mut GLchar);
            panic!("{}",
                   str::from_utf8(&buf)
                       .ok()
                       .expect("ShaderInfoLog not valid utf8"));
        }
    }
    shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
        // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(program,
                                  len,
                                  ptr::null_mut(),
                                  buf.as_mut_ptr() as *mut GLchar);
            panic!("{}",
                   str::from_utf8(&buf)
                       .ok()
                       .expect("ProgramInfoLog not valid utf8"));
        }
        program
    }
}