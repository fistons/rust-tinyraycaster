# [ssloy's tinyraycaster](https://github.com/ssloy/tinyraycaster/wiki/) - Rust edition

Poor attempt to rewrite this tiny raycaster in rust.

## Why?

I wanted a little pet project to learn Rust on my spare time, and I always
loved old Doom-like (as we used to say in the time). One day on hackernews, I 
found an article about ssloy's tiny raycaster, a small project to demostrate
how a raycasting engine, like the one in Wolfenstein 3D and Spear of Destiny,
works. It is a C++/SDL project under 500 lines of code, with a wiki to explain
how it works.

I though it was a good idea to translate it in rust to:
 1. Enhance my rust-fu
 2. Try to understand how a raycasting engine works

So thanks [ssloy](https://github.com/ssloy/) for the idea, the project and the inspiration!

## Requirements

Under Linux, You only need:
  * sdl2 (`apt-get install libsdl2-dev` with debian based distribution)
  * a rust toolchain (check https://rustup.rs/)

For Windows or Mac OS, I can't test

## How to run it

  * `git clone git@github.com:fistons/rust-tinyraycaster.git`
  * `cd rust-tinyraycaster`
  * `cargo run --release`

Without `--release`, the game will release in debug mode and will be slow as
hell.
