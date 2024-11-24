Implementation of Naderi Yeganeh's art in Rust

Ideas:

- [ ] Consider using `rayon` instead of `std::thread`
- [ ] Consider using `winit` `UserEvent` instead of `std::mpsc::channel`
- [ ] Consider porting to `rust-gpu`
- [ ] Position of cursor gives you the history of that value (i.e. all the computations that led to that pixel)
- [ ] proc-macro to write more math-like expressions, which will auto-generate the functions (and metadata needed for history)
- [ ] sunflower_field optimisation: v is always a constant integer

Workings are excalidraw-embedded png files
