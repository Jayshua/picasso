use gl;
use gl::types::*;

use std::mem;
use std::ptr;
use std::str;
use std::os::raw::c_void;
use std::ffi::CString;
use super::canvas::{Canvas, Fill};


// Shader sources
static VS_SRC: &'static str = r#"
   #version 150

   // Uniforms
   uniform vec2 viewsize;
   uniform vec2 point_a;
   uniform vec2 point_b;

   // Input
   in vec2 location;

   // Output
   out VS_OUT {
      vec2 location;
      vec2 original_location;
      vec2 point_a;
      vec2 point_b;
   } vs_out;

   // Prototypes
   vec2 transform_point(vec2 point);


   void main() {
      vs_out.original_location = location;
      vs_out.location = transform_point(location);
      vs_out.point_a  = transform_point(point_a);
      vs_out.point_b  = transform_point(point_b);

      gl_Position = vec4(transform_point(location).xy, 0.0, 1.0);
   }


   // Convert a point in pixel coordinates (from 0 to viewsize.xy)
   // to a point in OpenGL coordinates (from -1.0 to 1.0)
   vec2 transform_point(vec2 point) {
      return vec2(
          ((point.x / viewsize.x) * 2.0 - 1.0),
         -((point.y / viewsize.y) * 2.0 - 1.0)
      );
   }
"#;

static FS_SRC: &'static str = r#"
   #version 150

   // Uniforms
   uniform int fill_type;
   uniform vec4 color_a;
   uniform vec4 color_b;
   uniform vec2 point_a;
   uniform vec2 point_b;
   uniform sampler2D texture_a;

   // Input
   in VS_OUT {
      vec2 location;
      vec2 original_location;
      vec2 point_a;
      vec2 point_b;
   } fs_in;

   // Output
   out vec4 out_color;


   void main() {
      // Solid Color
      if (fill_type == 1) {
         out_color = color_a;
      }

      // Gradient
      else if (fill_type == 2) {
         vec2 difference = fs_in.point_b - fs_in.point_a;

         float multiplier = dot(fs_in.location - fs_in.point_a, normalize(difference)) / length(difference);

         out_color = vec4(
            color_a.r + multiplier * (color_b.r - color_a.r),
            color_a.g + multiplier * (color_b.g - color_a.g),
            color_a.b + multiplier * (color_b.b - color_a.b),
            color_a.a + multiplier * (color_b.a - color_a.a)
         );
      }

      // Image
      else if (fill_type == 3) {
         vec2 texture_location = vec2(
            (fs_in.original_location.x - point_a.x) / point_b.x,
            (fs_in.original_location.y - point_a.y) / point_b.y
         );

         out_color = texture(texture_a, texture_location);
      }

      // This shouldn't happen. Output a truly awful green color for debugging purposes.
      else {
         out_color = vec4(0.3, 1.0, 0.0, 1.0);
      }
   }
"#;


pub struct Renderer {
   vao: GLuint,
   vbo: GLuint,
   program: GLuint,
}



impl Drop for Renderer {
   fn drop(&mut self) {
      unsafe {
         println!("Dropping Shader Program");
         gl::DeleteProgram(self.program);
         gl::DeleteBuffers(1, &self.vbo);
         gl::DeleteVertexArrays(1, &self.vao);
      }
   }
}



impl Renderer {
   pub fn new() -> Renderer {
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

         Renderer {
            vao: vao,
            vbo: vbo,
            program: program,
         }
      }
   }



   pub fn draw_canvas(&self, window_width: u16, window_height: u16, canvas: &Canvas) {
      // Drawing a figure takes two steps. First, the figure is drawn to the
      // stencil buffer. Second, the figure is drawn to the color buffer. This
      // invokes some neat geometric sorcery that allows drawing concave
      // polygons without first triangulating them for OpenGL.
      // See the webpage below for more information on the technique.
      // http://what-when-how.com/opengl-programming-guide/drawing-filled-concave-polygons-using-the-stencil-buffer-opengl-programming/
      unsafe {
         let (points_buffer, points_buffer_length) = canvas.get_points_buffer();

         // Activate the buffer that stores the canvas's points
         gl::BindVertexArray(self.vao);

         // Activate the vector drawing program
         gl::UseProgram(self.program);

         // Upload the canvas's points to the GPU
         gl::BufferSubData(gl::ARRAY_BUFFER, 0, points_buffer_length as isize, points_buffer as *const c_void);

         // Tell the GPU how big the window is so that it can convert pixel coordinates into OpenGL coordinates
         gl::Uniform2f(self.get_uniform_location("viewsize"), window_width as f32, window_height as f32);

         // Invoke the sorcery of Geometry!
         gl::Enable(gl::STENCIL_TEST);

         // Draw each figure in the canvas
         for figure in canvas.figures_iter() {
            // Tell the GPU what type of fill to use
            match figure.fill {
               Fill::SolidColor((red, green, blue, alpha)) => {
                  let fill_type = self.get_uniform_location("fill_type");
                  let color_a = self.get_uniform_location("color_a");
                  gl::Uniform1i(fill_type, 1);
                  gl::Uniform4f(color_a, red, green, blue, alpha);
               },

               Fill::LinearGradient(begin, end, begin_color, end_color) => {
                  let fill_type = self.get_uniform_location("fill_type");
                  let color_a = self.get_uniform_location("color_a");
                  let color_b = self.get_uniform_location("color_b");
                  let point_a = self.get_uniform_location("point_a");
                  let point_b = self.get_uniform_location("point_b");
                  gl::Uniform1i(fill_type, 2);
                  gl::Uniform4f(color_a, begin_color.0, begin_color.1, begin_color.2, begin_color.3);
                  gl::Uniform4f(color_b, end_color.0, end_color.1, end_color.2, end_color.3);
                  gl::Uniform2f(point_a, begin.x, begin.y);
                  gl::Uniform2f(point_b, end.x, end.y);
               },

               Fill::Image(ref image, location, width, height) => {
                  let fill_type = self.get_uniform_location("fill_type");
                  let image_a = self.get_uniform_location("image_a");
                  let point_a = self.get_uniform_location("point_a");
                  let point_b = self.get_uniform_location("point_b");
                  gl::Uniform1i(fill_type, 3);
                  gl::Uniform1i(image_a, image.texture_id as i32);
                  gl::Uniform2f(point_a, location.x, location.y);
                  gl::Uniform2f(point_b, width, height);
               },
            }

            // Draw each path in the figure to the buffer
            for &(path_index, path_length) in &figure.paths {
               // First draw to the stencil buffer so that concave shapes appear correctly.
               // It's possible to optimize this call away for convex polygons. Someone should do this at some point.
               gl::StencilMask(0xff);
               gl::StencilFunc(gl::ALWAYS, 0, 0xff);
               gl::StencilOp(gl::INVERT, gl::INVERT, gl::INVERT);
               gl::ColorMask(gl::FALSE, gl::FALSE, gl::FALSE, gl::FALSE);
               gl::DrawArrays(gl::TRIANGLE_FAN, path_index as i32, path_length as i32);

               // Draw to the color buffer
               gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);
               gl::StencilFunc(gl::EQUAL, 0xff, 0xff);
               gl::StencilOp(gl::KEEP, gl::KEEP, gl::KEEP);
               gl::DrawArrays(gl::TRIANGLE_FAN, path_index as i32, path_length as i32);
            }
         }

         gl::Disable(gl::STENCIL_TEST);
      }
   }


   fn get_uniform_location(&self, name_str: &str) -> GLint {
      unsafe {
         let name = CString::new(name_str.as_bytes()).unwrap();
         gl::GetUniformLocation(self.program, name.as_ptr())
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
