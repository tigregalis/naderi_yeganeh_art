Implementation of [Hamid Naderi Yeganeh](https://x.com/naderi_yeganeh)'s art in Rust.

```sh
cargo run
cargo run --bin sunflower_field
```

Implemented:

- [Sunflower Field](https://x.com/naderi_yeganeh/status/1858455441782534161) ([Workings](./workings/sunflower_field.png))

Ideas:

- [x] Consider using `rayon` instead of `std::thread`
- [x] ~~Consider using `winit` `UserEvent` instead of `std::mpsc::channel`~~
- [ ] Consider porting to `rust-gpu`
- [ ] Position of cursor gives you a popup of the history of that value (i.e. all the computations that led to the colour of that pixel)
- [ ] Write a proc-macro to write more math-like expressions, which will auto-generate the functions (and the metadata needed for the history)
- [x] `sunflower_field` optimisation: `v` is always a constant integer

Workings are PNG images with the [Excalidraw](https://excalidraw.com) scene embedded into them.

# Decoding the math

## `v` or the colour component

`v` appears to be used, in many of the formulas (starting with `H(v, x, y)`), as a way to select a value for a colour, and is `0` for _red_, `1` for _green_, and `2` for _blue_.
So we need to find an equation such that `f(0) = r`, `f(1) = g`, `f(2) = b`.
A parabola is one equation with this behaviour.
We have the generalised formula for a parabola `f(v) = mv^2 + nv + o`.

We can solve for `m`, `n` and `o`:

```rs
f(0) = r
-> 0m + 0n + o = r
-> o = r

f(1) = g
-> 1m + 1n + r = g
-> m + n = -r + g

f(2) = b
-> 4m + 2n + r = b
-> 4m + 2n = -r + b

f(2) - 2f(1)
-> (4 - 2)m + (2 - 2)n = -r + b - 2(-r + g)
-> 2m + 0n = -r + b + 2r - 2g
-> 2m = (r - 2g + b)
-> m = (r - 2g + b) / 2

m + n = -r + g
-> n = -r + g - m
-> 2n = -2r + 2g - 2m
-> 2n = -2r + 2g -(r - 2g + b)
-> 2n = -2r + 2g - r + 2g - b
-> 2n = -3r + 4g - b
-> n = (-3r + 4g - b) / 2
```

Testing this equation:

```rs
r = 70% = 70/100
g = 70% = 70/100
b = 100% = 100/100

m = (r - 2g + b) / 2
-> m = (70 - 2*70 + 100) / 100 / 2
-> m = 30/200
-> m = 3/20

n = (-3r + 4g - b) / 2
-> n = (-3*70 + 4*70 - 100) / 100 / 2
-> n = -30/200
-> n = -3/20

o = r
-> o = 100/100
-> o = 20/20

f(v) = 3/20*v^2 - 3/20*v + 20/20
-> f(v) = (3*v^2 - 3*v + 20) / 20
```

This is the equation for the colour of the sky in `sunflower_field` `H(v, x, y)`
