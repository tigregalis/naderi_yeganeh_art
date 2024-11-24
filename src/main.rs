//! https://x.com/naderi_yeganeh/status/1858455441782534161/photo/1
//! Sunflower Field by Hamid Naderi Yeganeh

#![feature(fn_traits)]
#![feature(unboxed_closures)]
#![feature(tuple_trait)]

use std::{
    num::NonZeroU32,
    rc::Rc,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::{Duration, Instant},
};

use softbuffer::Surface;
use sunflower_field::*;
use utils::*;
use winit::{
    event::{Event, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{Key, ModifiersState, NamedKey},
    window::Window,
};

const FULL_M: usize = 2000;
const FULL_N: usize = 1200;
const HALF_M: f64 = (FULL_M / 2) as f64;
const HALF_N: f64 = (FULL_N / 2) as f64;
const HALF_N_PLUS_ONE: f64 = HALF_N + 1.;

mod memo;
mod winit_app;

struct State {
    pane: Pane,
    key_modifiers: ModifiersState,
    image: Vec<u32>,
    rx: Receiver<WorkerMessage>,
    threads: Vec<(usize, Sender<MainMessage>, u32)>,
    next_index: usize,
    last_tick: Instant,
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

#[derive(Debug)]
enum WorkerMessage {
    Idle { thread_id: usize },
    PixelReady { index: usize, pixel: u32 },
}

#[derive(Debug)]
enum MainMessage {
    Close,
    RequestPixels { index: usize, length: usize },
}
fn main() {
    let event_loop = EventLoop::new().unwrap();

    let app = winit_app::WinitAppBuilder::with_init(move |elwt| {
        let image = vec![u32::MAX; FULL_M * FULL_N];

        let (parent_tx, parent_rx) = mpsc::channel();
        let thread_count = thread::available_parallelism().unwrap().get() - 1;
        dbg!(thread_count);
        let threads = (0..thread_count)
            .map(move |thread_id| {
                let parent_tx = parent_tx.clone();
                let parent_tx_outer = parent_tx.clone();
                let (tx, rx) = mpsc::channel();
                thread::spawn(move || {
                    while let Ok(message) = rx.recv() {
                        match message {
                            MainMessage::Close => {
                                return;
                            }
                            MainMessage::RequestPixels { index, length } => {
                                if length == 0 {
                                    return;
                                }
                                // println!("received index {index}, length {length}");
                                // let compute_time = Instant::now();
                                for index in index..(index + length) {
                                    let (x, y) = xy_from_index(FULL_M, index);
                                    let m = (x + 1) as f64;
                                    let n = (y + 1) as f64;
                                    let rgb = draw(m, n);
                                    let pixel = softbuffer_color(rgb);
                                    if parent_tx
                                        .send(WorkerMessage::PixelReady { index, pixel })
                                        .is_err()
                                    {
                                        println!("channel closed");
                                        return;
                                    }
                                }
                                if parent_tx.send(WorkerMessage::Idle { thread_id }).is_err() {
                                    println!("channel closed");
                                    return;
                                }
                            }
                        }
                    }
                });

                if parent_tx_outer
                    .send(WorkerMessage::Idle { thread_id })
                    .is_err()
                {
                    println!("channel closed");
                }
                let placeholder_color = thread_id as f32 / thread_count as f32 * 255.;
                let placeholder_color = placeholder_color as u8;
                let placeholder_color =
                    softbuffer_color((placeholder_color, 255 - placeholder_color, 255));
                (thread_id, tx, placeholder_color)
            })
            .collect::<Vec<_>>();

        let next_index = 0;

        State {
            pane: Pane::new(elwt),
            key_modifiers: Default::default(),
            image,
            rx: parent_rx,
            threads,
            next_index,
            last_tick: Instant::now(),
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
            threads,
            next_index,
            last_tick,
            drawn,
        } = state;

        let image_len = image.len();

        while let Ok(message) = rx.try_recv() {
            match message {
                WorkerMessage::PixelReady { index, pixel } => {
                    if let Some(target) = image.get_mut(index) {
                        *target = pixel;
                        *drawn += 1;
                    }
                }
                WorkerMessage::Idle { thread_id: thread } => {
                    const LENGTH: usize = 512;
                    let (thread_original, tx, placeholder_color) = &threads[thread];
                    assert_eq!(*thread_original, thread);

                    let message = if *next_index < image_len {
                        image[*next_index..image_len.min(*next_index + LENGTH)]
                            .fill(*placeholder_color);
                        MainMessage::RequestPixels {
                            index: *next_index,
                            length: LENGTH,
                        }
                    } else {
                        MainMessage::Close
                    };
                    *next_index += LENGTH;
                    if tx.send(message).is_err() {
                        unreachable!("main thread does not initiate, only responds");
                    }
                }
            }
        }

        assert!(*drawn < *next_index);

        if last_tick.elapsed() >= Duration::from_secs_f64(1. / 240.) {
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
                        let target_width = width.min(FULL_M);
                        let target_height = height.min(FULL_N);
                        let src_buffer = image.as_mut_slice();
                        let src_lines = src_buffer
                            .chunks(FULL_M)
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

mod sunflower_field;
mod utils;
