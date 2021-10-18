#[cfg(feature = "ffi")]
extern crate cbindgen;

use std::{fmt::Write as FmtWrite, io::Write, fs::{ OpenOptions } , path::PathBuf};

use cbindgen::{Builder, Language};

fn main() {
    let out_dir = PathBuf::from("target");
    #[cfg(feature = "ffi")]
    {
        Builder::new()
            .with_crate("./")
            .with_language(Language::C)
            .generate()
            .expect("Unable to generate bindings")
            .write_to_file(out_dir.join("wrap.h"));
        Builder::new()
            .with_crate("./")
            .with_language(Language::Cxx)
            .generate()
            .expect("Unable to generate bindings")
            .write_to_file(out_dir.join("wrap.hpp"));
    }
}