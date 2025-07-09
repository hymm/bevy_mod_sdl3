mod keyboard;
mod non_send_marker;
mod window;

use std::{cell::RefCell, collections::HashMap, error::Error};

use bevy_app::{App, AppExit, Last, Plugin, PluginsState};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    entity::{Entity, EntityHashMap},
    error::BevyError,
    system::{Commands, Query},
};
use bevy_input::ButtonState;
use bevy_window::{RawHandleWrapper, RawHandleWrapperHolder, Window, WindowWrapper};
use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle,
};
use sdl3::{
    Sdl,
    event::{Event as SdlEvent, WindowEvent as SdlWindowEvent},
    video::Window as Sdl3Window,
};
use tracing::info;

use crate::{
    keyboard::handle_keyboard_events,
    non_send_marker::NonSendMarker,
    window::{Sdl3Windows, create_windows, handle_window_events},
};

pub struct Sdl3Plugin;
impl Plugin for Sdl3Plugin {
    fn build(&self, app: &mut bevy_app::App) {
        SdlContext::init();
        app.set_runner(sdl3_runner);
        app.add_systems(Last, create_windows);
    }
}

fn sdl3_runner(mut app: App) -> AppExit {
    if app.plugins_state() == PluginsState::Ready {
        app.finish();
        app.cleanup();
    }

    let mut event_pump = SDL_CONTEXT
        .with_borrow_mut(|sdl_context| sdl_context.as_mut().unwrap().sdl.event_pump())
        .unwrap();

    'running: loop {
        if app.plugins_state() != PluginsState::Cleaned {
            app.finish();
            app.cleanup();
        }

        for event in event_pump.poll_iter() {
            match event {
                SdlEvent::Window {
                    timestamp,
                    window_id,
                    win_event,
                } => {
                    handle_window_events(timestamp, window_id, win_event);
                }
                SdlEvent::KeyDown {
                    timestamp,
                    window_id,
                    keycode,
                    scancode,
                    keymod,
                    repeat,
                    which,
                    raw,
                } => handle_keyboard_events(
                    app.world_mut(),
                    ButtonState::Pressed,
                    timestamp,
                    window_id,
                    keycode,
                    scancode,
                    keymod,
                    repeat,
                    which,
                    raw,
                ),
                SdlEvent::KeyUp {
                    timestamp,
                    window_id,
                    keycode,
                    scancode,
                    keymod,
                    repeat,
                    which,
                    raw,
                } => handle_keyboard_events(
                    app.world_mut(),
                    ButtonState::Released,
                    timestamp,
                    window_id,
                    keycode,
                    scancode,
                    keymod,
                    repeat,
                    which,
                    raw,
                ),
                SdlEvent::Quit { .. } => {
                    break 'running;
                }
                _e => {
                    // dbg!(_e);
                }
            }
        }

        if app.plugins_state() == PluginsState::Cleaned {
            app.update();
        }
    }

    AppExit::Success
}

thread_local! {
    static SDL_CONTEXT: RefCell<Option<SdlContext>> = RefCell::new(None);
}

pub struct SdlContext {
    sdl: Sdl,
    windows: Sdl3Windows,
}

impl SdlContext {
    /// should be only called on the main thread
    fn init() {
        SDL_CONTEXT.with_borrow_mut(|context| {
            *context = Some(Self {
                sdl: sdl3::init().unwrap(),
                windows: Sdl3Windows::new(),
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
