# Rust kernel demo project

Based on the following tutorial:\
https://os.phil-opp.com/

### Dependencies

```
rustup component add rust-src
rustup override set nightly
rustup component add llvm-tools-preview
cargo install bootimage
```

### Build and run manually

Build the kernel and bootloader into a bootable image:

```
cargo bootimage
```

The bootable image can be found at `./target/x86_64-unknown-none/debug/bootimage-kernel.bin` and can be run manually:

```
qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-kernel.bin
```

### Run

```
cargo run
```
