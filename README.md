# Nove

**Nove** is a NES emulator started as a project on the **Open Source Club** at [Chaitin School](https://chaitinschool.org/). 
The core will be developed with Rust and it will features two different frontends: a desktop version running on SDL2
and one web frontend via WebAssembly (hopefully).

## Current state

- [x] The emulator can run the [6502 snake demo](http://skilldrick.github.io/easy6502/#snake)
  - [x] All legal op codes of the 6502
- [ ] The emulator completes [Kevin Horton's nestest](https://github.com/christopherpow/nes-test-roms/blob/master/other/nestest.txt)
  - [ ] All unofficial op codes are implemented  
- [ ] The emulator runs NES games demo mode 
- [ ] The emulator supports non-scrolling games (#canrunpacman)
- [ ] The emulator supports scrolling games (#canrunmariobros)
- [ ] The emulator is available on web

## How to run

The emulator is not yet ready, but it can be executed in some forms to check the current progress.

### Snake demo

The snake demo is a little snake game developed in assembly for the 6502 chip and it's usually a good way of checking
the implementation of the original CPU. _This is not a NES game_. An example featuring it can be run in desktop:

```shell
cargo run --example snake_demo
```

### CPU trace

The CPU supports tracing, and it can be enabled with setting the `TRACE` level to the `cpu` target.
This type of execution is essential for CPU testing, like checking against the `nestest`. An example is provided with
the core to run the CPU in this mode. You just need to provide the ROM path like this:

```shell
RUST_LOG=cpu=trace cargo run --quiet --example cpu_trace -- ./roms/nestest.nes
```