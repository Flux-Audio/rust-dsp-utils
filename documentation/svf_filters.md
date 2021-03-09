# State Variable Filters
Here the algorithms for the state variable filters in the library are explained
and illustrated in SPRING code.

## Pre-Processing
Finding frequency control coefficient `g` from cutoff frequency `F_c` and sampling
frequency `F_s`.
````R
f: float = 2.0*sin(pi*F_c / F_s);
````

Finding Q-factor `q` from resonance `res`
````R
q: float = (1.0 - res)*2.0;
````

## Filtering
Lowpass filter:
````R
lp: float = (bp@1)*f + lp@1;
````

Highpass filter:
````R
hp: float = input - lp - q*(bp@1);
````

Notch filter:
````R
notch: float = hp + lp;
````

Now we can find the bandpass:
````R
bp: float = hp*f + bp@1;
````

## Raw SPRING Code
````R
# res must be within [0, 1]
svf: fn = (input: float, cutoff: float, res: float, sr: float)
       -> (lp: float, hp: float, bp: float, notch: float) {
    # Pre-process
    f: float = 2.0*sin(pi*cutoff / sr);
    q: float = (1.0 - res)*2.0;

    # Filtering
    lp:    out = (bp@1)*f + lp@1;
    hp:    out = input - lp - q*(bp@1);
    notch: out = hp + lp;
    bp:    out = hp*f + bp@1;
}
````

## Rust Translation
````rust
// State variables, translated from the SPRING delay operator
struct Svf {
    lp_z1:    f32,
    hp_z1:    f32,
    notch_z1: f32,
    bp_z1:    f32,
}

impl Svf {
    // init
    fn new() -> Self {
        Self {
            lp_z1:    0.0,
            hp_z1:    0.0,
            notch_z1: 0.0,
            bp_z1:    0.0,
        }
    }

    fn compute(input: f32, cutoff: f32, res: f32, sr: f32) -> (f32, f32, f32, f32) {
        // Pre-process
        let f = 2.0 * (f32::consts::PI*cutoff / sr).sin();
        let q = (1.0 - res)*2.0;

        // Filtering
        let lp    = bp_z1*f + lp_z1;
        let hp    = input - lp - q*bp_z1;
        let notch = hp + lp;
        let bp    = hp*f + bp_z1;

        // Update state:
        self.lp_z1    = lp;
        self.hp_z1    = hp;
        self.notch_z1 = notch;
        self.bp_z1    = bp;

        // return 
        return (lp, hp, notch, bp);
    }
}

````