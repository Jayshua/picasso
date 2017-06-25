extern crate gl;
extern crate glutin;

mod shader_program;
mod point;
mod figure;


use shader_program::ShaderProgram;
use figure::Figure;






fn main() {
    let events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_multisampling(4)
        .build(&events_loop)
        .unwrap();

    // It is essential to make the context current before calling `gl::load_with`.
    unsafe { window.make_current() }.unwrap();
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);


    let figure: Figure = Figure::new()
        .rectangle(-1.0, -1.0, 2.0, 2.0)
        .fill_linear_gradient(-1.0, 0.0, 1.0, 0.0, (0.76, 0.49, 0.69, 1.0), (0.08, 0.0, 0.06, 1.0));

    let program = ShaderProgram::new();


    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent {event: glutin::WindowEvent::Closed, ..} => running = false,
                _ => ()
            }
        });


        unsafe {
            // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            // Clear the screen to black
            gl::ClearColor(0.9, 0.9, 0.9, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
            program.draw_figure(&figure);
        }

        window.swap_buffers().unwrap();
    }

    program.drop();
}

