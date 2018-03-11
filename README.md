seccomp-droundy-sys
-------------------

Raw bindings to libseccomp. Does not require
[libseccomp](https://github.com/seccomp/libseccomp) to be installed,
but instead compiles it from source if it is not installed.

This is a fork of https://github.com/polachok/seccomp-sys.

This library provides a high level interface to constructing, analyzing and installing seccomp filters via a BPF passed to the Linux Kernel's prctl() syscall.

