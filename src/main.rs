extern crate sdl2;
extern crate gl;

fn main() {

    //SDL init
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let _window = video_subsystem
        .window("CalcSFC Indev", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    //GL init
    let gl_context = window.gl_create_context().unwrap();

    //Defining event pump
    let mut event_pump = sdl.event_pump().unwrap();

    //Main loop
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _=> {},
            }
        }

        // render window contents here
    }
}
