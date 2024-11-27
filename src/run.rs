use std::{
    num::NonZeroU32,
    rc::Rc,
    sync::mpsc::{self, Receiver},
    time::{Duration, Instant},
};

use crate::{utils::*, winit_app, Art};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use softbuffer::Surface;
use winit::{
    event::{Event, KeyEvent, WindowEvent},
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
                            Key::Named(NamedKey::ArrowLeft) => {}
                            Key::Named(NamedKey::ArrowRight) => {}
                            Key::Named(NamedKey::ArrowUp) => {}
                            Key::Named(NamedKey::ArrowDown) => {}
                            Key::Named(NamedKey::Escape) => {}
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
                WindowEvent::CursorMoved { .. } => {}
                WindowEvent::MouseInput { .. } => {}
                WindowEvent::MouseWheel { .. } => {}
                WindowEvent::RedrawRequested => {
                    let width = surface.window().inner_size().width as usize;
                    let height = surface.window().inner_size().height as usize;
                    let mut surface_buffer = surface.buffer_mut().unwrap();

                    if width > 0 && height > 0 {
                        let target_width = width.min(Artwork::FULL_M);
                        let target_height = height.min(Artwork::FULL_N);
                        let src_buffer = image.as_mut_slice();
                        let src_lines = src_buffer
                            .chunks(Artwork::FULL_M)
                            .map(|line| &line[0..target_width])
                            .take(target_height);
                        let dest_buffer = surface_buffer.as_mut();
                        dest_buffer.fill(0);
                        let dest_lines = dest_buffer
                            .chunks_mut(width)
                            .map(|line| &mut line[0..target_width])
                            .take(target_height);
                        for (src, dest) in src_lines.zip(dest_lines) {
                            dest.copy_from_slice(src);
                        }
                    }

                    surface.buffer_mut().unwrap().present().unwrap();
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
