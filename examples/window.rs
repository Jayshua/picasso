use glutin;
use glutin::GlContext;
use gl;

// Create and return a window with an active OpenGL Context
// This part has nothing to do with Picasso, you can use whatever
// windowing library you like. Glutin is used here, which is an
// excellent choice if you are looking for one.
pub fn create_window() -> (glutin::EventsLoop, glutin::GlWindow) {
   let events_loop = glutin::EventsLoop::new();

   let window = glutin::WindowBuilder::new()
      .with_title("Picasso Rectangles")
      .with_dimensions(800, 400);

   let context = glutin::ContextBuilder::new()
      .with_vsync(true);

   let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

   unsafe {
      gl_window.make_current().unwrap();
      gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
   }

   (events_loop, gl_window)
}