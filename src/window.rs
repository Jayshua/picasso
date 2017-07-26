use ::glutin;
use ::gl;

use super::shader_program::ShaderProgram;
use super::canvas::Canvas;


/// Represents the window that appears to the user.
/// Create one of these than pass it a canvas to render using the `render` method.
/// Be sure to periodically check the `running` attribute to see if the user
/// has closed the application.
pub struct Window {
   pub running: bool,
   events_loop: glutin::EventsLoop,
   window: glutin::Window,
   shader_program: ShaderProgram,
}



impl Window {
   /// Construct a new Picasso Window
   pub fn new() -> Window {
      // Build the glutin window
      let events_loop = glutin::EventsLoop::new();
      let glutin_window = glutin::WindowBuilder::new()
         .with_multisampling(2)
         .build(&events_loop)
         .unwrap();

      // Make the window's OpenGl context current
      unsafe { glutin_window.make_current().unwrap(); }

      // Tell the GL loader where to load OpenGL Functions from
      gl::load_with(|symbol| glutin_window.get_proc_address(symbol) as *const _);

      Window {
         events_loop: events_loop,
         window: glutin_window,
         shader_program: ShaderProgram::new(),
         running: true,
      }
   }




   /// Render the provided canvas to the window.
   /// Note: Events are polled in this method, so the window will
   /// not be able to close if you only call it rarely.
   pub fn render(&mut self, canvas: &Canvas) {
      // Check if the window should close
      let mut running = self.running;
      self.events_loop.poll_events(|event| {
         match event {
            glutin::Event::WindowEvent {event: glutin::WindowEvent::Closed, ..} => running = false,
            _ => ()
         }
      });
      self.running = running;

      // Clear the screen
      unsafe {
         gl::ClearColor(0.9, 0.9, 0.3, 1.0);
         gl::Clear(gl::COLOR_BUFFER_BIT);
      }

      // Render the requested canvas
      self.shader_program.draw_canvas(&canvas);

      // Swap the buffers so the image actually appears
      self.window.swap_buffers().unwrap();
   }
}