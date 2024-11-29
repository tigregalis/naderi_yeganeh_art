use std::{
    fmt::Write as _,
    num::NonZeroU32,
    rc::Rc,
    sync::mpsc::{self, Receiver},
    time::{Duration, Instant},
};

use crate::{utils::*, winit_app, Art};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use softbuffer::Surface;
use track::{set_should_track, with_stack};
use winit::{
    event::{ElementState, Event, KeyEvent, MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{Key, ModifiersState, NamedKey},
    window::Window,
};

struct State {
    pane: Pane,
    key_modifiers: ModifiersState,
    image: Vec<u32>,
    rx: Receiver<PixelReady>,
    last_tick: Instant,
    time_started: Instant,
    finished: bool,
    drawn: usize,
    mouse: Mouse,
    scroll_x: i32,
    scroll_y: i32,
}

/// A [`winit::window::Window`] paired with a [`softbuffer::Surface`]
struct Pane<D = Rc<Window>, W = D> {
    window: W,
    surface: Surface<D, W>,
}

impl Pane {
    fn new(elwt: &ActiveEventLoop) -> Self {
        let window = winit_app::make_window(elwt, |w| w);
        let context = softbuffer::Context::new(window.clone()).unwrap();
        let surface = Surface::new(&context, window.clone()).unwrap();
        Self { window, surface }
    }
}

const BATCH_SIZE: usize = 32;

#[derive(Debug)]
struct PixelReady {
    index: usize,
    pixels: [u32; BATCH_SIZE],
}

struct Mouse {
    prev_x: f64,
    prev_y: f64,
    x: f64,
    y: f64,
    left_state: ElementState,
    middle_state: ElementState,
}

impl Default for Mouse {
    fn default() -> Self {
        Self {
            prev_x: Default::default(),
            prev_y: Default::default(),
            x: Default::default(),
            y: Default::default(),
            left_state: ElementState::Released,
            middle_state: ElementState::Released,
        }
    }
}

pub fn run<Artwork: Art>() {
    let event_loop = EventLoop::new().unwrap();

    let app = winit_app::WinitAppBuilder::with_init(move |elwt| {
        let image = vec![u32::MAX; Artwork::FULL_M * Artwork::FULL_N];

        let (tx, rx) = mpsc::channel();

        rayon::spawn(move || {
            (0..((Artwork::FULL_M * Artwork::FULL_N).div_ceil(BATCH_SIZE)))
                .into_par_iter()
                .for_each_with(tx, |tx, counter| {
                    let mut pixels = [u32::MAX; BATCH_SIZE];
                    let index = counter * BATCH_SIZE;
                    for (offset, pixel) in pixels.iter_mut().enumerate() {
                        let (x, y) = xy_from_index(Artwork::FULL_M, index + offset);
                        let m = (x + 1) as f64;
                        let n = (y + 1) as f64;
                        let rgb = Artwork::draw(m, n);
                        *pixel = softbuffer_color(rgb);
                    }

                    // debug_print_stored_values();

                    if tx.send(PixelReady { index, pixels }).is_err() {
                        eprintln!("loop no longer exists");
                    }
                });
        });

        State {
            pane: Pane::new(elwt),
            key_modifiers: Default::default(),
            image,
            rx,
            last_tick: Instant::now(),
            time_started: Instant::now(),
            finished: false,
            drawn: 0,
            mouse: Default::default(),
            scroll_x: 0,
            scroll_y: 0,
        }
    })
    .with_event_handler(|state, event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);

        let State {
            pane: Pane { window, surface },
            key_modifiers,
            image,
            rx,
            last_tick,
            time_started,
            finished,
            drawn,
            mouse,
            scroll_x,
            scroll_y,
        } = state;

        let image_len = image.len();

        while let Ok(PixelReady { index, pixels }) = rx.try_recv() {
            let len = (index + BATCH_SIZE).min(image_len) - index;
            image[index..(index + len)].copy_from_slice(&pixels);
            *drawn += len;
        }

        if !*finished && *drawn >= image_len {
            *finished = true;
            println!("Finished in {elapsed:?}", elapsed = time_started.elapsed());
        }

        let refresh_rate = window
            .current_monitor()
            .and_then(|mon| mon.refresh_rate_millihertz())
            .unwrap_or(60000);
        if last_tick.elapsed() >= Duration::from_secs_f64(1000. / refresh_rate as f64) {
            window.request_redraw();
            *last_tick = Instant::now();
        }

        match event {
            Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
                WindowEvent::Resized(size) => {
                    if let (Some(x_len), Some(y_len)) =
                        (NonZeroU32::new(size.width), NonZeroU32::new(size.height))
                    {
                        surface.resize(x_len, y_len).unwrap();
                    }
                }
                WindowEvent::ModifiersChanged(modifiers) => {
                    *key_modifiers = modifiers.state();
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    let KeyEvent {
                        logical_key, state, ..
                    } = event;

                    if state.is_pressed() {
                        match logical_key {
                            Key::Named(NamedKey::ArrowLeft) => {
                                *scroll_x -= 20;
                            }
                            Key::Named(NamedKey::ArrowRight) => {
                                *scroll_x += 20;
                            }
                            Key::Named(NamedKey::ArrowUp) => {
                                *scroll_y -= 20;
                            }
                            Key::Named(NamedKey::ArrowDown) => {
                                *scroll_y += 20;
                            }
                            Key::Named(NamedKey::Escape) => {
                                *scroll_x = 0;
                                *scroll_y = 0;
                            }
                            Key::Named(NamedKey::Enter) => {}
                            Key::Named(NamedKey::Space) => {}
                            Key::Named(key) => {
                                if let Some(text) = key.to_text() {
                                    for _c in text.chars() {}
                                }
                            }
                            Key::Character(text) => {
                                if !key_modifiers.control_key() {
                                    for _c in text.chars() {}
                                }
                            }
                            _ => {}
                        }
                    };
                }
                WindowEvent::CursorMoved {
                    device_id: _,
                    position,
                } => {
                    mouse.prev_x = mouse.x;
                    mouse.prev_y = mouse.y;

                    mouse.x = position.x;
                    mouse.y = position.y;

                    let dx = mouse.x - mouse.prev_x;
                    let dy = mouse.y - mouse.prev_y;

                    if mouse.middle_state == ElementState::Pressed {
                        *scroll_x += dx as i32;
                        *scroll_y += dy as i32;
                    }
                }
                WindowEvent::MouseInput {
                    device_id: _,
                    state,
                    button,
                } => {
                    if button == MouseButton::Left {
                        let x = mouse.x as isize - *scroll_x as isize;
                        let y = mouse.y as isize - *scroll_y as isize;
                        if state == ElementState::Released
                            && x >= 0
                            && y >= 0
                            && x < Artwork::FULL_M as isize
                            && y < Artwork::FULL_N as isize
                        {
                            struct Temp {
                                lines: Vec<(usize, String)>,
                                callstack: Vec<usize>,
                            }

                            impl Temp {
                                fn current(&mut self) -> &'_ mut String {
                                    &mut self.lines[*self.callstack.last().unwrap()].1
                                }
                            }

                            let mut tmp = Temp {
                                lines: Vec::new(),
                                callstack: Vec::new(),
                            };

                            std::thread::spawn(move || {
                                set_should_track(true);

                                let m = (x + 1) as f64;
                                let n = (y + 1) as f64;
                                let rgb = Artwork::draw(m, n);
                                println!("@ m = {m}, n = {n}");

                                set_should_track(false);
                                with_stack(|stack| -> Result<(), std::fmt::Error> {
                                    tmp.lines.reserve(stack.len() / 4); // Minimum items is: Start, Arg* (since always >= 1 arg), ArgEnd, Finish*
                                    tmp.callstack.reserve(stack.len() / 4); // Minimum items is: Start, Arg* (since always >= 1 arg), ArgEnd, Finish*
                                    let mut depth = 0usize;
                                    for item in stack.drain(..) {
                                        match item {
                                            track::Item::Start(name) => {
                                                tmp.lines.push((depth, "  ".repeat(depth)));
                                                tmp.callstack.push(tmp.lines.len() - 1);
                                                write!(tmp.current(), "{name}(")?;
                                                depth += 1;
                                            }
                                            track::Item::ArgUsize(arg, val) => {
                                                write!(tmp.current(), "{arg} = {val}, ")?;
                                            }
                                            track::Item::ArgF64(arg, val) => {
                                                write!(tmp.current(), "{arg} = {val:.3}, ")?;
                                            }
                                            track::Item::ArgEnd => {
                                                tmp.current().pop(); // remove extra " "
                                                tmp.current().pop(); // remove extra ","
                                                write!(tmp.current(), ")")?;
                                            }
                                            track::Item::FinishRgb(r, g, b) => {
                                                write!(tmp.current(), " = ({r}, {g}, {b})")?;
                                                tmp.callstack.pop();
                                                depth -= 1;
                                            }
                                            track::Item::FinishF64(output) => {
                                                write!(tmp.current(), " = {output:.3}")?;
                                                tmp.callstack.pop();
                                                depth -= 1;
                                            }
                                        }
                                    }
                                    for (_, line) in tmp.lines.drain(..) {
                                        println!("  {line}");
                                    }
                                    Ok(())
                                })
                                .expect("Writing to a string should have succeeded");

                                println!("=> rgb({}, {}, {})", rgb.0, rgb.1, rgb.2);
                            });
                        }

                        mouse.left_state = state;
                    } else if button == MouseButton::Middle {
                        mouse.middle_state = state;
                    } else if button == MouseButton::Right && state == ElementState::Pressed {
                        *scroll_x = 0;
                        *scroll_y = 0;
                    }
                }
                WindowEvent::MouseWheel {
                    device_id: _,
                    delta,
                    phase: _,
                } => {
                    let (offset_x, offset_y) = match delta {
                        winit::event::MouseScrollDelta::LineDelta(x, y) => {
                            (x as i32 * 100, y as i32 * 100)
                        }
                        winit::event::MouseScrollDelta::PixelDelta(pos) => {
                            (pos.x as i32, pos.y as i32)
                        }
                    };
                    *scroll_x += if key_modifiers.shift_key() {
                        offset_y
                    } else {
                        offset_x
                    };
                    *scroll_y += if key_modifiers.shift_key() {
                        offset_x
                    } else {
                        offset_y
                    };
                }
                WindowEvent::RedrawRequested => {
                    let width = surface.window().inner_size().width as isize;
                    let height = surface.window().inner_size().height as isize;
                    let mut surface_buffer = surface.buffer_mut().unwrap();

                    if width > 0 && height > 0 {
                        let start_x = *scroll_x as isize;
                        let skip_dest_x = start_x.max(0).min(width) as usize;
                        let skip_src_x = (-start_x).max(0).min(Artwork::FULL_M as isize) as usize;
                        let take_x = (width - skip_dest_x as isize)
                            .min(Artwork::FULL_M as isize - skip_src_x as isize)
                            .max(0) as usize;

                        let start_y = *scroll_y as isize;
                        let skip_dest_y = start_y.max(0).min(height) as usize;
                        let skip_src_y = (-start_y).max(0).min(Artwork::FULL_N as isize) as usize;
                        let take_y = (height - skip_dest_y as isize)
                            .min(Artwork::FULL_N as isize - skip_src_y as isize)
                            .max(0) as usize;

                        let src_buffer = image.as_slice();
                        let src_lines = src_buffer
                            .chunks(Artwork::FULL_M)
                            .skip(skip_src_y)
                            .map(|line| &line[skip_src_x..(skip_src_x + take_x)])
                            .take(take_y);
                        let dest_buffer = surface_buffer.as_mut();
                        dest_buffer.fill(0);
                        let dest_lines = dest_buffer
                            .chunks_mut(width as usize)
                            .map(|line| &mut line[skip_dest_x..(skip_dest_x + take_x)])
                            .skip(skip_dest_y)
                            .take(take_y);
                        for (src, dest) in src_lines.zip(dest_lines) {
                            dest.copy_from_slice(src);
                        }
                    }

                    surface_buffer.present().unwrap();
                }
                WindowEvent::CloseRequested => {
                    elwt.exit();
                }
                _ => {}
            },
            _ => {}
        }
    });
    winit_app::run_app(event_loop, app);
}
