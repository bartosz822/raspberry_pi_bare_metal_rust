# Raspberry PI Bare Metal Rust
### Barotsz Radzyński Małgorzata Stachoń

Simple raspberry pi 3 LED blinking written in Rust, working without OS

### Requirements

* xargo
* arm-none-eabi toolchain

### Compilation

Siply type `make` in terminal

### Running

In order to run this copy `kernel.img` to first partition of your Raspberry PI sd card.
`bootcode.bin` and `start.elf` also need to be present in this location.
