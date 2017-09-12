extern crate pkg_config;
extern crate gcc;

use std::io::{Read,Write};

fn main() {

    if pkg_config::probe_library("libseccomp").is_err() {
        // libseccomp is not installed as a system library... We thus
        // need to build it from source.
        if !std::path::Path::new("libseccomp/.git").exists() {
            let _ = std::process::Command::new("git")
                .args(&["submodule", "update", "--init"])
                .status();
        }

        let src_dir = std::path::Path::new("libseccomp/src");

        // I manually determine which files to compile, and then use
        // the gcc crate, which *does* know how to cross-compile
        // properly to do the actual building of the library.
        let mut sources = Vec::new();
        for f in src_dir.read_dir() {
            for f in f.flat_map(|f| f.ok()) { // lazy way to ignore errors
                if f.path().extension() == Some(std::ffi::OsStr::new("c"))
                    && f.file_name() != std::ffi::OsStr::new("arch-syscall-check.c")
                    && f.file_name() != std::ffi::OsStr::new("arch-syscall-dump.c") {
                        sources.push(f.path());
                    }
            }
        }
        // I create an empty file named config.h, since libseccomp
        // doesn't actually require anything in the config.h, just
        // that it exist.
        let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
        std::fs::File::create(out_dir.join("configure.h")).unwrap();

        let mut header_in = std::fs::File::open(src_dir.join("../include/seccomp.h.in")).unwrap();
        let mut contents = String::new();
        header_in.read_to_string(&mut contents)
            .expect("something went wrong reading the seccomp.h.in");
        let contents = contents.replace("@VERSION_MAJOR@", "2");
        let contents = contents.replace("@VERSION_MINOR@", "3");
        let contents = contents.replace("@VERSION_MICRO@", "2");
        {
            let mut header_out = std::fs::File::create(out_dir.join("seccomp.h")).unwrap();
            write!(header_out, "{}", contents).unwrap();
        }

        gcc::Build::new()
            .files(sources)
            .include(&out_dir)
            .include(&src_dir)
            .include(src_dir.join(".."))
            .include(src_dir.join("../include"))
            .compile("libseccomp.a");
    }
}
