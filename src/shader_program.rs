use gl;
use gl::types::*;

use std::mem;
use std::ptr;
use std::str;
use std::os::raw::c_void;
use std::ffi::CString;
use super::canvas::Canvas;


// Shader sources
static VS_SRC: &'static str = r#"
   #version 150

   in vec2 position;

   void main() {
      gl_Position = vec4(position, 0.0, 1.0);
   }
"#;

static FS_SRC: &'static str = r#"
   #version 150

   out vec4 out_color;
   uniform vec4 color;

   void main() {
      out_color = color;
   }
"#;


pub struct ShaderProgram {
   vao: GLuint,
   vbo: GLuint,
   program: GLuint,
}



impl Drop for ShaderProgram {
   fn drop(&mut self) {
      unsafe {
         println!("Dropping Shader Program");
         gl::DeleteProgram(self.program);
         gl::DeleteBuffers(1, &self.vbo);
         gl::DeleteVertexArrays(1, &self.vao);
      }
   }
}



impl ShaderProgram {
   pub fn new() -> ShaderProgram {
      let program = link_program(VS_SRC, FS_SRC);

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
            (100 * mem::size_of::<(f32, f32)>()) as GLsizeiptr,
            ptr::null(),
            gl::STATIC_DRAW
         );

         // Use shader program
         gl::UseProgram(program);

         // Specify the layout of the vertex data
         gl::EnableVertexAttribArray(0);
         gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());

         gl::BindVertexArray(0);

         ShaderProgram {
            vao: vao,
            vbo: vbo,
            program: program,
         }
      }
   }



   pub fn draw_canvas(&self, canvas: &Canvas) {
      unsafe {
         let (points_buffer, points_buffer_length) = canvas.get_points_buffer();

         // Activate the vertex buffer
         gl::BindVertexArray(self.vao);

         // Activate the vector drawing program
         gl::UseProgram(self.program);

         // Upload the points to the GPU
         gl::BufferSubData(gl::ARRAY_BUFFER, 0, points_buffer_length as isize, points_buffer as *const c_void);

         // Draw each figure in the canvas
         for figure in canvas.figures_iter() {
            let (red, green, blue, alpha) = figure.fill;
            let color_str = CString::new("color".as_bytes()).unwrap();
            gl::Uniform4f(
               gl::GetUniformLocation(self.program, color_str.as_ptr()),
               red, green, blue, alpha
            );

            // Draw each path in the figure to the buffer
            for &(path_index, path_length) in &figure.paths {
               gl::DrawArrays(gl::TRIANGLE_FAN, path_index as i32, path_length as i32);
            }
         }
      }
   }
}






fn compile_shader(src: &str, ty: GLenum) -> GLuint {
   unsafe {
      let shader = gl::CreateShader(ty);

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

      shader
   }
}




fn link_program(vs: &str, fs: &str) -> GLuint {
   unsafe {
      // Build the program
      let program = gl::CreateProgram();
      let vertex_shader = compile_shader(vs, gl::VERTEX_SHADER);
      let fragment_shader = compile_shader(fs, gl::FRAGMENT_SHADER);
      gl::AttachShader(program, vertex_shader);
      gl::AttachShader(program, fragment_shader);
      gl::LinkProgram(program);

      // Cleanup the shaders used
      gl::DeleteShader(vertex_shader);
      gl::DeleteShader(fragment_shader);

      // Check if the build was successful
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
