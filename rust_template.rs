// <editor-fold> Preamble: usefull functions for IO
use std::io;

#[allow(dead_code)]
fn read_i32() -> i32 {
    let mut input_buffer = String::new();
    io::stdin().read_line(&mut input_buffer).ok().expect("read error");
    return input_buffer.trim().parse().ok().expect("parse error");
}

#[allow(dead_code)]
fn read_f32() -> f32 {
    let mut input_buffer = String::new();
    io::stdin().read_line(&mut input_buffer).ok().expect("read error");
    return input_buffer.trim().parse().ok().expect("parse error");
}

#[allow(dead_code)]
fn read_str() -> String {
    let mut input_buffer = String::new();
    io::stdin().read_line(&mut input_buffer).ok().expect("read error");
    return input_buffer;
}

#[allow(dead_code)]
fn read_vec() -> Vec<i32> {
    let array_string = read_str();
    return array_string.split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<i32>>();
}

#[allow(dead_code)]
fn print_vec(v: &Vec<i32>) {
    print!("[");
    for x in v {
        print!("{} ", x)
    }
    println!("]")
}
// </editor-fold>

fn main() {
    
}
