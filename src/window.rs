extern crate sdl2;
    use sdl2::*;
extern crate gl;

pub fn get_window() -> (video::Window, video::GLContext, EventPump) {
    //SDL init
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("CalcSFC Indev", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    //GL init
    let gl_context = window.gl_create_context().unwrap();

    //Defining event pump
    let event_pump = sdl.event_pump().unwrap();

    (window, gl_context, event_pump)
}