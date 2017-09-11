extern crate pkg_config;
extern crate curl;
extern crate gcc;

use std::io::{Write};

use curl::easy::Easy;

fn main() {

    if pkg_config::probe_library("ibseccomp").is_err() {
        // libseccomp is not installed as a system library... ideally
        // we would like to build it from source, to make installation
        // easier.
        let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

        let mut tarball = std::fs::File::create(out_dir.join("libseccomp.tar.gz")).unwrap();
        let mut handle = Easy::new();
        handle.follow_location(true).unwrap();
        handle.url("https://github.com/seccomp/libseccomp/releases/download/v2.3.2/libseccomp-2.3.2.tar.gz").unwrap();
        handle.write_function(move |data| {
            Ok(tarball.write(data).unwrap())
        }).unwrap();
        handle.perform().unwrap();

        run("Trouble untarring source code",
            std::process::Command::new("tar")
            .args(&["xzf", "libseccomp.tar.gz"])
            .current_dir(&out_dir));

        let build_dir = out_dir.join("libseccomp-2.3.2");
        let src_dir = build_dir.join("src");

        // The following is an unholy hodge-podge of techniques.  I
        // use ./configure to generate the "config.h" file, which is
        // used by the library.  But sadly, I can't seem to get make
        // to cross-compile libseccomp properly, even though I
        // specified the --host argument to ./configure.  So instead,
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
        let prefix = out_dir.join("libseccomp");

        let target = std::env::var("TARGET").unwrap();
        println!("target is {}", target);
        run("Trouble configuring source code",
            std::process::Command::new("./configure")
            .args(&["--enable-shared=no", "--disable-dependency-tracking",
                    "--prefix",])
            .arg(prefix)
            .arg("--host")
            .arg(target)
            .current_dir(&build_dir));

        gcc::Build::new()
            .files(sources)
            .include(&src_dir)
            .include(&build_dir)
            .include(build_dir.join("include"))
            .compile("libseccomp.a");
    }
}

fn run(error_msg: &'static str, cmd: &mut std::process::Command) {
    if !cmd.status().expect(error_msg).success() {
        panic!(error_msg);
    }
}
