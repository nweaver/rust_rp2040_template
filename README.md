Basic skeleton of a Rust project for Raspberry Pi Pico/RP2040
microcontrollers, from

https://www.fullstacklabs.co/blog/rust-raspberry-pi-pico-blink

This one is modified slightly to blink the 3-color LEDs on the Xiao
SEEED RP2040 miniature board.


Packages I needed to install on Ubuntu

sudo apt install cmake gcc-arm-none-eabi libnewlib-arm-none-eabi build-essential minicom libudev-dev


Some setup on rust is required:

rustup target add thumbv6m-none-eabi
cargo install cargo-binutils
rustup component add llvm-tools-preview
cargo install elf2uf2-rs


I haven't tested this with the two pico board setup, instead just
creating a uf2 file that can be successfully flashed onto a RP2040 in
boot mode.
