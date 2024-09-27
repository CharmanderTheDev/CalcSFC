extern crate sdl2;
extern crate gl;

pub mod window;
pub mod functionreader;

fn main() {
    let function = ['1','2','3','4','.','0','1'];
    let result = functionreader::number_parse(&function).unwrap().0;
    println!("{result}");

    //let (window, gl, eventpump) = window::get_window();
}
