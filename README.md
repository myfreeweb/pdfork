# pdfork

A Rust fork wrapper that uses process descriptors (pdfork) on FreeBSD and normal fork elsewhere.

Process descriptors are like file descriptors but for processes:
- they are immune to PID race conditions (they track the exact process in the kernel);
- they work in the [Capsicum](https://wiki.freebsd.org/Capsicum) capability mode sandbox.

```rust
extern crate libc;
extern crate pdfork;
use pdfork::*;

match fork() {
    ForkResult::Fail => panic!("fork"),
    ForkResult::Parent(child_proc) => {
        // do stuff
        // you can access child_proc.child_pid on any platform
        // you can also access child_proc.child_pd on FreeBSD
        if !child_proc.signal(libc::SIGTERM) {
            panic!("sigterm");
        }
    },
    ForkResult::Child => {
        // do stuff
    }
}
```

## Contributing

By participating in this project you agree to follow the [Contributor Code of Conduct](https://www.contributor-covenant.org/version/1/4/).

[The list of contributors is available on GitHub](https://github.com/myfreeweb/pdfork/graphs/contributors).

## License

This is free and unencumbered software released into the public domain.  
For more information, please refer to the `UNLICENSE` file or [unlicense.org](http://unlicense.org).
