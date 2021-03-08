# Flux-Audio Plugin Layout

## Local File Structure
```ps1
src/
├─── editor.rs      # GUI (with the druid crate)
├─── lib.rs         # main and business logic
└─── compute.rs     # utility function specific to the plugin
```
``lib.rs`` contains the plugin main and business logic.
``compute.rs`` contains additional helper function, to hide the implementation details from the plugin's main.

## Dependencies
Toml file:
```toml
dsp_utils = { git = "https://github.com/Flux-Audio/rust-dsp-utils" }
```

Rust source file:
````rust
// syntax
// use dsp_utils::<module name>;

// example:
use  dsp_utils::{effects::saturation, fft::windows};
````


## Utils Repository Structure
````
src/
├─── lib.rs                 # contains crate root and unit tests
├─── vst.rs                 # contains helper macros for the rust-vst library
├─── utils/
|    ├─── mod.rs
│    ├─── math.rs           # various mathematical functions, like normalization, median, ...
│    ├─── chaos.rs          # various random value generators
│    └─── delay.rs          # delay lines
├─── effects/
|    ├─── mod.rs
│    ├─── saturation.rs     # various saturation functions, soft-clip, ...
│    ├─── hysteresis.rs     # various hysteresis functions, like dx^3, ...
│    └─── bias.rs           # various bias functions, like swish, ReLU, ...
└─── fft/
     ├─── mod.rs
     └─── windows.rs        # various windowing functions for fft
````