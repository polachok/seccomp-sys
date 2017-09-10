extern crate pkg_config;

fn main() {

    if pkg_config::probe_library("libseccomp").is_err() {
        // libseccomp is not installed as a system library... ideally
        // we would like to build it from source, to make installation
        // easier.
        panic!("Need libseccomp library (with headers) to be installed!");
    }
}
