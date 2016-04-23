use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::path::Path;
use glium::program::{Program, ProgramCreationError};
use glium::backend::Facade;


pub fn load_shader_source<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}

pub fn create_shader_program<P: AsRef<Path>, F>(display: &F, vertex_path: P, fragment_path: P, geometry_path: Option<P>) -> Result<Program, ProgramCreationError> where F: Facade {
    let vertex_source = load_shader_source(vertex_path).unwrap();
    let fragment_source = load_shader_source(fragment_path).unwrap();
    let source;
    let geometry_source = match geometry_path {
        None => None,
        Some(path) => {
            source = load_shader_source(path).unwrap();
            Some(source.as_str())
        },
    };

    Program::from_source(display, vertex_source.as_str(), fragment_source.as_str(), geometry_source)
}
