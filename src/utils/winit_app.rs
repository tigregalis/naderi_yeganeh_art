//! Common boilerplate for setting up a winit application.
//! Adapted from <https://github.com/rust-windowing/softbuffer/blob/6d65b5bb6046a175ad271972d7848b6185ef5d96/examples/utils/winit_app.rs>

use std::marker::PhantomData;
use std::rc::Rc;

use winit::application::ApplicationHandler;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

/// Run a Winit application.
#[allow(unused_mut)]
pub fn run_app<E>(event_loop: EventLoop<E>, mut app: impl ApplicationHandler<E> + 'static) {
    #[cfg(not(any(target_arch = "wasm32", target_arch = "wasm64")))]
    event_loop.run_app(&mut app).unwrap();

    #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
    winit::platform::web::EventLoopExtWebSys::spawn_app(event_loop, app);
}

/// Create a window from a set of window attributes.
#[allow(dead_code)]
pub fn make_window(
    elwt: &ActiveEventLoop,
    f: impl FnOnce(WindowAttributes) -> WindowAttributes,
) -> Rc<Window> {
    let attributes = f(WindowAttributes::default());
    #[cfg(target_arch = "wasm32")]
    let attributes = winit::platform::web::WindowAttributesExtWebSys::with_append(attributes, true);
    let window = elwt.create_window(attributes);
    Rc::new(window.unwrap())
}

/// Easily constructable winit application.
pub struct WinitApp<T, Init, Handler, E> {
    /// Closure to initialize state.
    init: Init,

    /// Closure to run on window events.
    event: Handler,

    /// Contained state.
    state: Option<T>,

    _marker: PhantomData<E>,
}

/// Builder that makes it so we don't have to name `T`.
pub struct WinitAppBuilder<T, Init> {
    /// Closure to initialize state.
    init: Init,

    /// Eat the type parameter.
    _marker: PhantomData<Option<T>>,
}

impl<T, Init> WinitAppBuilder<T, Init>
where
    Init: FnMut(&ActiveEventLoop) -> T,
{
    /// Create with an "init" closure.
    pub fn with_init(init: Init) -> Self {
        Self {
            init,
            _marker: PhantomData,
        }
    }

    /// Build a new application.
    pub fn with_event_handler<F, E>(self, handler: F) -> WinitApp<T, Init, F, E>
    where
        F: FnMut(&mut T, Event<E>, &ActiveEventLoop),
    {
        WinitApp::new(self.init, handler)
    }
}

impl<T, Init, Handler, E> WinitApp<T, Init, Handler, E>
where
    Init: FnMut(&ActiveEventLoop) -> T,
    Handler: FnMut(&mut T, Event<E>, &ActiveEventLoop),
{
    /// Create a new application.
    pub fn new(init: Init, event: Handler) -> Self {
        Self {
            init,
            event,
            state: None,
            _marker: PhantomData,
        }
    }
}

impl<T, Init, Handler, E> ApplicationHandler<E> for WinitApp<T, Init, Handler, E>
where
    Init: FnMut(&ActiveEventLoop) -> T,
    Handler: FnMut(&mut T, Event<E>, &ActiveEventLoop),
    E: 'static,
{
    fn resumed(&mut self, el: &ActiveEventLoop) {
        debug_assert!(self.state.is_none());
        self.state = Some((self.init)(el));
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        let state = self.state.take();
        debug_assert!(state.is_some());
        drop(state);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let state = self.state.as_mut().unwrap();
        (self.event)(state, Event::WindowEvent { window_id, event }, event_loop);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(state) = self.state.as_mut() {
            (self.event)(state, Event::AboutToWait, event_loop);
        }
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: E) {
        let state = self.state.as_mut().unwrap();
        (self.event)(state, Event::UserEvent(event), event_loop);
    }
}