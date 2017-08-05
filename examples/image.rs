extern crate gl;
extern crate picasso;

extern crate glutin;
use glutin::GlContext;

mod window;


fn main() {
   // Create an OpenGL window
   let (mut events_loop, window) = window::create_window();

   let picasso_renderer = picasso::Renderer::new();
   // Careful! It's easy to unintentionally stretch the image if you provide the wrong width/height
   let path = picasso::Canvas::new()
      .rectangle(0.0, 0.0, 800.0, 400.0)
      .fill_image(picasso::Image::new("examples/oliver-hen-pritchard-barrett.jpg").unwrap(), 0.0, 0.0, 800.0, 400.0);


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
         gl::Clear(gl::COLOR_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
      }

      picasso_renderer.draw_canvas(800, 400, &path);
      window.swap_buffers().unwrap();
   }
}
