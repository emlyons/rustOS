# rust-os-cs140e
## An Experimental Course on Operating Systems

Assignments from the [CS140 course](https://cs140e.sergio.bz/).

### Directory Structure

```
.
├── bin : common binaries/utilities
├── doc : reference documents
├── ext : external files (e.g., resources for testing)
├── tut : tutorial/practices
│    ├── 0-rustlings
│    ├── 1-blinky
│    ├── 2-shell
│    └── 3-fs : questions for lab3 *
├── boot : bootloader
├── kern : the main os kernel *
└── lib  : required libraries
     ├── fat32 *
     ├── pi *
     ├── shim
     ├── stack-vec
     ├── ttywrite
     ├── volatile
     └── xmodem

```

### Rust Versioning
```
$ rustup install nightly-2018-01-09
$ rustup default nightly-2018-01-09
$ rustup override set nightly-2018-01-09
$ rustup component add rust-src

$ cargo install xargo --version 0.3.10

$ rustc --version
rustc 1.25.0-nightly (b5392f545 2018-01-08)

$ xargo --version
xargo 0.3.10
cargo 0.25.0-nightly (a88fbace4 2017-12-29)
```

## Bootstrapping Raspberry Pi
Phase 0 - 4 from [Assignment 0: Blinky](https://cs140e.sergio.bz/assignments/0-blinky/).
Get the enviornment setup and make and LED blink in C and Rust.

### Phase 0: Getting Started
- [x] Getting your Pi Ready
- [x] Getting the Skeleton Code

### Phase 1: Baking Pi
- [x] Installing Driver
- [x] Powering the Pi
- [x] Running Programs

### Phase 2: LED There Be Light
- [x] GPIO: General Purpose I/O
- [x] Testing the LED

### Phase 3: Shining C
- [x] Installing a Cross-Compiler
- [x] Talking to Hardware
- [x] GPIO Memory-Mapped Interface
- [x] Writing the Code

### Phase 4: Rusting Away
- [x] Installing Rust and Xargo
- [x] Writing the Code


## Shell and Bootloader
Phase 0 - 2 from [Assignment 1: Shell](https://cs140e.sergio.bz/assignments/1-shell/).
Write `stack-vec`, `volatile`, `ttywrite`, and `xmodem` libraries.

### Phase 0: Getting Started
- [x] Getting the Skeleton Code

### Phase 2: Oxidation
- [x] Subphase A: StackVec
- [x] Subphase B: volatile
- [x] Subphase C: xmodem
- [x] Subphase D: ttywrite

### Phase 3: *Not* a Seashell
- [x] Subphase A: Getting Started
- [x] Subphase B: System Timer
- [x] Subphase C: GPIO
- [x] Subphase D: UART
- [x] Subphase E: The Shell
     
### Phase 4: Boot 'em Up
- [x] Loading Binaries
- [x] Making Space
- [x] Implementing the Bootloader

## FAT32 Filesystem
Phase 0 - 4 from [Assignment 2: File System](https://cs140e.sergio.bz/assignments/2-fs/).


### Phase 0: Getting Started
- [x] Getting the Skeleton Code

### Phase 1: Memory Lane
- [x] Subphase A: Panic!
- [x] Subphase B: ATAGS
- [x] Subphase C: Warming Up
- [x] Subphase D: Bump Allocator
- [x] Subphase E: Bin Allocator

### Phase 2: 32-bit Lipids
- [x] Implementation

### Phase 3: Saddle Up
- [ ] Subphase A: SD Driver FFI
- [ ] Subphase B: File System

### Phase 4: Mo'sh
- [ ] Working Directory
- [ ] Commands
- [ ] Implementation

## Why Rust?

Historically, C has been mainly used for OS development because of its portability,
minimal runtime, direct hardware/memory access, and (decent) usability.
Rust provides all of these features with addition of memory safety guarantee,
strong type system, and modern language abstractions
which help programmers to make less mistakes when writing code.

## Acknowledgement

We built our labs based on the materials originally developed for
[CS140e: An Experimental Course on Operating Systems](https://cs140e.sergio.bz/)
by [Sergio Benitez](https://sergio.bz/).
We have ported it to use newer toolchains such as Rust 2018 edition,
`cargo-xbuild` (instead of `xargo`), and `no_std` Rust with a minimal shim library
(instead of custom built std).
We’ve also developed it further to include topics such as virtual memory management, multicore scheduling, mutex designing, and implementing a networking stack.
