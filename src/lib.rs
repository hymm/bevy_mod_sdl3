mod keyboard;
mod mouse;
mod non_send_marker;
mod window;

use std::cell::RefCell;

use bevy_app::{App, AppExit, Last, Plugin, PluginsState};
use bevy_ecs::entity::Entity;
use bevy_input::ButtonState;
use bevy_window::WindowEvent;
use sdl3::{Sdl, event::Event as SdlEvent};

use crate::{
    keyboard::handle_keyboard_events,
    mouse::{handle_mouse_button, handle_mouse_motion, handle_mouse_wheel},
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
                } => handle_window_events(app.world_mut(), timestamp, window_id, win_event),
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
                SdlEvent::MouseMotion {
                    timestamp: _,
                    window_id,
                    which: _,
                    mousestate: _,
                    x,
                    y,
                    xrel,
                    yrel,
                } => handle_mouse_motion(app.world_mut(), window_id, x, y, xrel, yrel),
                SdlEvent::MouseButtonDown {
                    timestamp: _,
                    window_id,
                    which: _,
                    mouse_btn,
                    clicks: _,
                    x: _,
                    y: _,
                } => handle_mouse_button(
                    app.world_mut(),
                    window_id,
                    mouse_btn.into(),
                    ButtonState::Pressed,
                ),
                SdlEvent::MouseButtonUp {
                    timestamp: _,
                    window_id,
                    which: _,
                    mouse_btn,
                    clicks: _,
                    x: _,
                    y: _,
                } => handle_mouse_button(
                    app.world_mut(),
                    window_id,
                    mouse_btn.into(),
                    ButtonState::Released,
                ),
                SdlEvent::MouseWheel {
                    timestamp: _,
                    window_id,
                    which: _,
                    x,
                    y,
                    direction,
                    // position on window
                    mouse_x: _,
                    mouse_y: _,
                } => handle_mouse_wheel(app.world_mut(), window_id, x, y, direction),
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
    static SDL_CONTEXT: RefCell<Option<SdlContext>>  = RefCell::new(None);
}

pub struct SdlContext {
    sdl: Sdl,
    windows: Sdl3Windows,
}

impl SdlContext {
    /// should be only called on the main thread
    fn init() {
        SDL_CONTEXT.with_borrow_mut(|context| {
            *context = Some(SdlContext {
                sdl: sdl3::init().unwrap(),
                windows: Sdl3Windows::new(),
            });
        });
    }

    fn get_window_entity(sdl_id: u32) -> impl FnOnce(&Option<SdlContext>) -> Option<Entity> {
        move |context| {
            context
                .as_ref()
                .unwrap()
                .windows
                .winit_to_entity
                .get(&sdl_id.into())
                .copied()
        }
    }

    fn get_window_entity_and_scale(
        sdl_id: u32,
    ) -> impl FnOnce(&Option<SdlContext>) -> Option<(Entity, f32)> {
        move |context: &Option<SdlContext>| {
            context.as_ref().and_then(|context| {
                let entity = context
                    .windows
                    .winit_to_entity
                    .get(&sdl_id.into())
                    .copied()?;
                let window = context.windows.get_window(entity)?;

                Some((entity, window.display_scale()))
            })
        }
    }
}
