# Flux-Audio Plugin Layout

## Local File Structure
```ps1
src/
├─── lib.rs
└─── compute.rs
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
````ps1
src/
├─── utils/
│    └─── math.rs           # various mathematical functions, like normalization, median, ...
├─── effects/
│    ├─── saturation.rs     # various saturation functions, like tanh(x), soft-clip, ...
│    ├─── hysteresis.rs     # various hysteresis functions, like dx^3, ...
│    └─── bias.rs           # various bias functions, like swish, ReLU, ...
└─── fft/
     └─── windows.rs        # various windowing functions for fft
````
