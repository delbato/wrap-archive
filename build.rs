#[cfg(feature = "ffi")]
extern crate cbindgen;

use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from("target");
    #[cfg(feature = "ffi")]
    {
        use cbindgen::{Builder, Language};

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
