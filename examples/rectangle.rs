extern crate gl;
extern crate picasso;

extern crate glutin;
use glutin::GlContext;

mod window;


fn main() {
   // Create an OpenGL window
   let (mut events_loop, window) = window::create_window();

   // Here's the important part
   // Picasso has two parts: A renderer and a canvas.
   // Canvases are used to create the art, and a renderer is used to put it on the screen.
   //
   // From a technical standpoint, (which is completely unnecessary to know)
   // the renderer keeps track of the OpenGL state required to perform the
   // rendering like references to the shaders and buffers.
   let picasso_renderer = picasso::Renderer::new();
   let green_rectangle = picasso::Canvas::new()
      .rectangle(0.0, 0.0, 0.5, 0.5)
      .fill(0.2, 0.8, 0.2, 1.0);

   let mut running = true;
   while running {
      // End the program if the user closes the window
      events_loop.poll_events(|event| {
         if let glutin::Event::WindowEvent { event: glutin::WindowEvent::Closed, .. } = event {
            running = false;
         }
      });

      unsafe {
         gl::ClearColor(0.9, 0.2, 0.2, 1.0);
         gl::Clear(gl::COLOR_BUFFER_BIT);
      }

      picasso_renderer.draw_canvas(&green_rectangle);
      window.swap_buffers().unwrap();
   }
}


