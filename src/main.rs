extern crate sdl2;
extern crate gl;

pub mod window;
pub mod functionreader;

fn main() {
    let function = ['s','i','n','(','4','x',')','9'];
    let result = functionreader::read_function(&function, 4.0).unwrap();
    println!("{result}");

    //let (window, gl, eventpump) = window::get_window();
}
