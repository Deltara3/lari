# Lari
An experimental programming language.

## building different binaries
normally, when using `cargo b`, every binary gets compiled in the same batch, if this isn't what you want, use this
```sh
$ cargo b --bin laric # build the lari compiler
$ cargo b --bin lariup # build the lari updater
$ cargo b --bin lariman # build the lari project manager
```