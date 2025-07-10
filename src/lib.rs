mod keyboard;
mod non_send_marker;
mod window;

use std::cell::RefCell;

use bevy_app::{App, AppExit, Last, Plugin, PluginsState};
use bevy_input::ButtonState;
use bevy_window::WindowEvent;
use sdl3::{Sdl, event::Event as SdlEvent};

use crate::{
    keyboard::handle_keyboard_events,
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
                    handle_window_events(app.world_mut(), timestamp, window_id, win_event);
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
                // TODO: we may need to do more with AppLifecyle to match the winit behavior
                SdlEvent::AppWillEnterBackground { timestamp: _ } => {
                    app.world_mut().send_event(WindowEvent::AppLifecycle(
                        bevy_window::AppLifecycle::WillSuspend,
                    ));
                }
                SdlEvent::AppDidEnterBackground { timestamp: _ } => {
                    app.world_mut().send_event(WindowEvent::AppLifecycle(
                        bevy_window::AppLifecycle::Suspended,
                    ));
                }
                SdlEvent::AppWillEnterForeground { timestamp: _ } => {
                    app.world_mut().send_event(WindowEvent::AppLifecycle(
                        bevy_window::AppLifecycle::WillResume,
                    ));
                }
                SdlEvent::AppDidEnterForeground { timestamp: _ } => {
                    app.world_mut().send_event(WindowEvent::AppLifecycle(
                        bevy_window::AppLifecycle::Running,
                    ));
                }
                // SdlEvent::AppTerminating { timestamp } => todo!(),
                // SdlEvent::AppLowMemory { timestamp } => todo!(),
                // SdlEvent::TextEditing {
                //     timestamp,
                //     window_id,
                //     text,
                //     start,
                //     length,
                // } => todo!(),
                // SdlEvent::TextInput {
                //     timestamp,
                //     window_id,
                //     text,
                // } => todo!(),
                // SdlEvent::MouseMotion {
                //     timestamp,
                //     window_id,
                //     which,
                //     mousestate,
                //     x,
                //     y,
                //     xrel,
                //     yrel,
                // } => todo!(),
                // SdlEvent::MouseButtonDown {
                //     timestamp,
                //     window_id,
                //     which,
                //     mouse_btn,
                //     clicks,
                //     x,
                //     y,
                // } => todo!(),
                // SdlEvent::MouseButtonUp {
                //     timestamp,
                //     window_id,
                //     which,
                //     mouse_btn,
                //     clicks,
                //     x,
                //     y,
                // } => todo!(),
                // SdlEvent::MouseWheel {
                //     timestamp,
                //     window_id,
                //     which,
                //     x,
                //     y,
                //     direction,
                //     mouse_x,
                //     mouse_y,
                // } => todo!(),
                // SdlEvent::JoyAxisMotion {
                //     timestamp,
                //     which,
                //     axis_idx,
                //     value,
                // } => todo!(),
                // SdlEvent::JoyHatMotion {
                //     timestamp,
                //     which,
                //     hat_idx,
                //     state,
                // } => todo!(),
                // SdlEvent::JoyButtonDown {
                //     timestamp,
                //     which,
                //     button_idx,
                // } => todo!(),
                // SdlEvent::JoyButtonUp {
                //     timestamp,
                //     which,
                //     button_idx,
                // } => todo!(),
                // SdlEvent::JoyDeviceAdded { timestamp, which } => todo!(),
                // SdlEvent::JoyDeviceRemoved { timestamp, which } => todo!(),
                // SdlEvent::ControllerAxisMotion {
                //     timestamp,
                //     which,
                //     axis,
                //     value,
                // } => todo!(),
                // SdlEvent::ControllerButtonDown {
                //     timestamp,
                //     which,
                //     button,
                // } => todo!(),
                // SdlEvent::ControllerButtonUp {
                //     timestamp,
                //     which,
                //     button,
                // } => todo!(),
                // SdlEvent::ControllerDeviceAdded { timestamp, which } => todo!(),
                // SdlEvent::ControllerDeviceRemoved { timestamp, which } => todo!(),
                // SdlEvent::ControllerDeviceRemapped { timestamp, which } => todo!(),
                // SdlEvent::ControllerTouchpadDown {
                //     timestamp,
                //     which,
                //     touchpad,
                //     finger,
                //     x,
                //     y,
                //     pressure,
                // } => todo!(),
                // SdlEvent::ControllerTouchpadMotion {
                //     timestamp,
                //     which,
                //     touchpad,
                //     finger,
                //     x,
                //     y,
                //     pressure,
                // } => todo!(),
                // SdlEvent::ControllerTouchpadUp {
                //     timestamp,
                //     which,
                //     touchpad,
                //     finger,
                //     x,
                //     y,
                //     pressure,
                // } => todo!(),
                // SdlEvent::FingerDown {
                //     timestamp,
                //     touch_id,
                //     finger_id,
                //     x,
                //     y,
                //     dx,
                //     dy,
                //     pressure,
                // } => todo!(),
                // SdlEvent::FingerUp {
                //     timestamp,
                //     touch_id,
                //     finger_id,
                //     x,
                //     y,
                //     dx,
                //     dy,
                //     pressure,
                // } => todo!(),
                // SdlEvent::FingerMotion {
                //     timestamp,
                //     touch_id,
                //     finger_id,
                //     x,
                //     y,
                //     dx,
                //     dy,
                //     pressure,
                // } => todo!(),
                // SdlEvent::DollarRecord {
                //     timestamp,
                //     touch_id,
                //     gesture_id,
                //     num_fingers,
                //     error,
                //     x,
                //     y,
                // } => todo!(),
                // SdlEvent::MultiGesture {
                //     timestamp,
                //     touch_id,
                //     d_theta,
                //     d_dist,
                //     x,
                //     y,
                //     num_fingers,
                // } => todo!(),
                // SdlEvent::ClipboardUpdate { timestamp } => todo!(),
                // SdlEvent::DropFile {
                //     timestamp,
                //     window_id,
                //     filename,
                // } => todo!(),
                // SdlEvent::DropText {
                //     timestamp,
                //     window_id,
                //     filename,
                // } => todo!(),
                // SdlEvent::DropBegin {
                //     timestamp,
                //     window_id,
                // } => todo!(),
                // SdlEvent::DropComplete {
                //     timestamp,
                //     window_id,
                // } => todo!(),
                // SdlEvent::AudioDeviceAdded {
                //     timestamp,
                //     which,
                //     iscapture,
                // } => todo!(),
                // SdlEvent::AudioDeviceRemoved {
                //     timestamp,
                //     which,
                //     iscapture,
                // } => todo!(),
                // SdlEvent::RenderTargetsReset { timestamp } => todo!(),
                // SdlEvent::RenderDeviceReset { timestamp } => todo!(),
                // SdlEvent::User {
                //     timestamp,
                //     window_id,
                //     type_,
                //     code,
                //     data1,
                //     data2,
                // } => todo!(),
                // SdlEvent::Unknown { timestamp, type_ } => todo!(),
                // SdlEvent::Display {
                //     timestamp,
                //     display,
                //     display_event,
                // } => todo!(),
                e => {
                    dbg!(e);
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
