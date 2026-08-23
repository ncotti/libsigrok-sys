# LIBSIGROK-SYS

This crate generates Rust FFI (Foreign Function Interface) bindings for the [libsigrok library][libsigrok_wiki] and the [libsigrokdecode library][libsigrokdecode_wiki] using [bindgen][bindgen].

For its usage, the `libsigrok` and `libsigrokdecode` must be installed in one of the following paths:

* `/usr/lib/<arch>-<OS>-<libc>` and `/usr/include/<arch>-<OS>-<libc>`.
* `/usr/local/lib` and `/usr/local/include`
* Any path set in the `LD_LIBRARY_PATH` env. variable.

The libraries can be installed with using a package manager as follows:

```bash
sudo apt install libsigrok-dev libsigrokdecode-dev
```

The `LD_LIBRARY_PATH` variable takes precedence over the system paths.

```toml
[dependencies]
libsigrok-sys = "x.x.x"
```

<!-- External links -->
[libsigrok_wiki]: https://sigrok.org/wiki/Libsigrok
[libsigrokdecode_wiki]: https://sigrok.org/wiki/Libsigrokdecode
[bindgen]: https://rust-lang.github.io/rust-bindgen/
