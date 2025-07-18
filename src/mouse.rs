use bevy_ecs::world::World;
use bevy_input::{
    ButtonState,
    mouse::{
        MouseButton as BevyMouseButton, MouseButtonInput, MouseMotion, MouseScrollUnit, MouseWheel,
    },
};
use bevy_math::Vec2;
use bevy_window::{CursorMoved, Window as BevyWindow};
use sdl3::mouse::{MouseButton as SdlMouseButton, MouseWheelDirection};

use crate::{SDL_CONTEXT, SdlContext};

pub fn handle_mouse_motion(
    world: &mut World,
    window_id: u32,
    x: f32,
    y: f32,
    xrel: f32,
    yrel: f32,
) {
    let (entity, scale) = SDL_CONTEXT
        .with_borrow(SdlContext::get_window_entity_and_scale(window_id))
        .unwrap();

    // Note that this is actually sending the accumulated mouse delta unlike winit
    world.send_event(MouseMotion {
        delta: Vec2::new(xrel, yrel),
    });

    let physical_position = Vec2::new(x, y);
    let logical_position = physical_position / scale;

    let mut entity_mut = world.get_entity_mut(entity).unwrap();
    let mut bevy_window = entity_mut.get_mut::<BevyWindow>().unwrap();

    let last_position = bevy_window.physical_cursor_position();
    let delta = last_position.map(|last_pos| (physical_position - last_pos) / scale);
    bevy_window.set_physical_cursor_position(Some(physical_position.into()));

    world.send_event(CursorMoved {
        window: entity,
        position: logical_position,
        delta,
    });
}

pub fn handle_mouse_button(
    world: &mut World,
    window_id: u32,
    button: SdlMouseButton,
    state: ButtonState,
) {
    let window = SDL_CONTEXT
        .with_borrow(SdlContext::get_window_entity(window_id))
        .unwrap();
    world.send_event(MouseButtonInput {
        button: convert_sdl_mouse_button(button),
        state,
        window,
    });
}

pub fn handle_mouse_wheel(
    world: &mut World,
    window_id: u32,
    x: f32, // positive to the right and negative to the left
    y: f32, // positive away from the user and negative toward the user
    _direction: MouseWheelDirection,
) {
    // TODO: figure out how to deal with flipped mouse wheel direction
    // TODO: get scrolled lines from sdl. not exposed by lib yet
    let window = SDL_CONTEXT
        .with_borrow(SdlContext::get_window_entity(window_id))
        .unwrap();
    world.send_event(MouseWheel {
        unit: MouseScrollUnit::Pixel,
        x,
        y,
        window,
    });
}

pub fn convert_sdl_mouse_button(sdl_button: SdlMouseButton) -> BevyMouseButton {
    match sdl_button {
        // TODO: should map other mouse buttons, should bevy have an unknown state?
        // need to add passing through to the sdl3 crate
        SdlMouseButton::Unknown => BevyMouseButton::Other(0),
        SdlMouseButton::Left => BevyMouseButton::Left,
        SdlMouseButton::Middle => BevyMouseButton::Middle,
        SdlMouseButton::Right => BevyMouseButton::Right,
        SdlMouseButton::X1 => BevyMouseButton::Back,
        SdlMouseButton::X2 => BevyMouseButton::Forward,
    }
}
