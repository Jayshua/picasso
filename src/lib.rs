/*!
Provides very basic OpenGL backed drawing commands.

# Examples

```
use picasso::{Window, Canvas};

fn main() {
    let mut window = Window::new();

    let first_rect: Canvas = Canvas::new()
        .move_to(0.0, 0.0)
        .line_to(0.5, 0.0)
        .line_to(0.5, 0.5)
        .line_to(0.0, 0.5)
        .fill(1.0, 0.0, 0.0, 1.0);

    let second_rect: Canvas = Canvas::new()
        .move_to(-0.1, -0.1)
        .line_to(-0.5, -0.1)
        .line_to(-0.5, -0.5)
        .line_to(-0.1, -0.5)
        .fill(0.0, 1.0, 0.0, 1.0);

    let both_rects: Canvas = Canvas::new()
         .attach(&first_rect)
         .attach(&second_rect);

    while window.running {
        window.render(&both_rects);
    }
}
```
*/


extern crate gl;
extern crate glutin;

mod canvas;
mod shader_program;
mod window;

pub use self::canvas::Canvas;
pub use self::window::Window;