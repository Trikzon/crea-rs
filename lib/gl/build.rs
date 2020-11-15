extern crate gl_generator;

use gl_generator::{
    Registry, Fallbacks, StructGenerator,
    DebugStructGenerator, Api, Profile,
};
use std::{env, fs::File};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let mut bindings_file = File::create("./src/bindings.rs").unwrap();
    let mut debug_bindings_file = File::create("./src/debug_bindings.rs").unwrap();

    let registry = Registry::new(Api::Gl, (4, 5), Profile::Core, Fallbacks::All, [
        "GL_NV_command_list", // additional extensions we want to use
    ]);

    // if env::var("CARGO_FEATURE_DEBUG").is_ok() {
        registry.write_bindings(
            DebugStructGenerator, // different generator
            &mut debug_bindings_file
        ).unwrap();
    // } else {
        registry.write_bindings(
            StructGenerator, // different generator
            &mut bindings_file
        ).unwrap();
    // }
}
