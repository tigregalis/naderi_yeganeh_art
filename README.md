Implementation of [Hamid Naderi Yeganeh](https://x.com/naderi_yeganeh)'s art in Rust.

Implemented:

- [Sunflower Field](https://x.com/naderi_yeganeh/status/1858455441782534161) ([Workings](./workings/sunflower_field.png))

Ideas:

- [ ] Consider using `rayon` instead of `std::thread`
- [ ] Consider using `winit` `UserEvent` instead of `std::mpsc::channel`
- [ ] Consider porting to `rust-gpu`
- [ ] Position of cursor gives you a popup of the history of that value (i.e. all the computations that led to the colour of that pixel)
- [ ] Write a proc-macro to write more math-like expressions, which will auto-generate the functions (and the metadata needed for the history)
- [x] sunflower_field optimisation: v is always a constant integer

Workings are PNG images with the [Excalidraw](https://excalidraw.com) scene embedded into them.
