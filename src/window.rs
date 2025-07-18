use std::{collections::HashMap, error::Error};

use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    entity::{Entity, EntityHashMap},
    error::BevyError,
    system::{Commands, Query},
    world::World,
};
use bevy_math::IVec2;
use bevy_window::{
    CursorEntered, CursorLeft, RawHandleWrapper, RawHandleWrapperHolder, Window,
    WindowCloseRequested, WindowFocused, WindowMoved, WindowOccluded, WindowResized, WindowTheme,
    WindowWrapper,
};
use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle,
};
use sdl3::{Sdl, VideoSubsystem, event::WindowEvent, video::Window as Sdl3Window};
use tracing::info;

use crate::{SDL_CONTEXT, SdlContext, non_send_marker::NonSendMarker};

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct WindowId(pub u32);

impl From<u32> for WindowId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

#[derive(Deref, DerefMut)]
pub struct SyncWindow(Sdl3Window);

// TODO: not sure if this is safe. example only does this for &Sdl3Window. It might be that that is a hack for wgpu.
unsafe impl<'a> Send for SyncWindow {}
unsafe impl<'a> Sync for SyncWindow {}

impl HasWindowHandle for SyncWindow {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        self.0.window_handle()
    }
}

impl HasDisplayHandle for SyncWindow {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        self.0.display_handle()
    }
}

/// A resource mapping window entities to their `sdl3`-backend [`Window`](winit::window::Window)
/// states.
#[derive(Default)]
pub struct Sdl3Windows {
    /// Stores [`winit`] windows by window identifier.
    pub windows: HashMap<WindowId, WindowWrapper<SyncWindow>>,
    /// Maps entities to `sdl3` window identifiers.
    pub entity_to_winit: EntityHashMap<WindowId>,
    /// Maps `sdl3` window identifiers to entities.
    pub winit_to_entity: HashMap<WindowId, Entity>,
    // Many `winit` window functions (e.g. `set_window_icon`) can only be called on the main thread.
    // If they're called on other threads, the program might hang. This marker indicates that this
    // type is not thread-safe and will be `!Send` and `!Sync`.
    _not_send_sync: core::marker::PhantomData<*const ()>,
}

impl Sdl3Windows {
    pub fn new() -> Self {
        Self {
            windows: HashMap::default(),
            entity_to_winit: EntityHashMap::new(),
            winit_to_entity: HashMap::new(),
            _not_send_sync: std::marker::PhantomData,
        }
    }

    fn create_window(
        &mut self,
        sdl: &Sdl,
        entity: Entity,
        bevy_window: &Window,
    ) -> Result<&WindowWrapper<SyncWindow>, Box<dyn Error + Send + Sync>> {
        let video = sdl.video()?;
        let sdl_window = video
            .window(
                bevy_window.name.as_ref().unwrap_or(&"Bevy".to_string()),
                bevy_window.width() as u32,
                bevy_window.height() as u32,
            )
            .position_centered()
            .resizable()
            .metal_view()
            .build()
            .map_err(|e| e.to_string())?;
        let id = WindowId(sdl_window.id());
        self.windows
            .insert(id, WindowWrapper::new(SyncWindow(sdl_window)));
        self.entity_to_winit.insert(entity, id);
        self.winit_to_entity.insert(id, entity);

        Ok(self.windows.get(&id).unwrap())
    }

    pub fn get_window(&self, entity: Entity) -> Option<&WindowWrapper<SyncWindow>> {
        let id = self.entity_to_winit.get(&entity)?;
        self.windows.get(id)
    }
}

/// system to create the windows when a Window is spawned
pub fn create_windows(
    mut commands: Commands,
    mut created_windows: Query<(Entity, &mut Window, Option<&RawHandleWrapperHolder>)>,
    // sdl windows need to be created on the main thread
    _non_send: NonSendMarker,
) -> Result<(), BevyError> {
    SDL_CONTEXT.with_borrow_mut(|context| {
        let context = context.as_mut().ok_or(BevyError::from(
            "Sdl3 Context not found. Did you forget to call init()",
        ))?;

        for (entity, mut window, handle_holder) in &mut created_windows {
            if context.windows.get_window(entity).is_some() {
                continue;
            }

            info!("Creating new window {} ({})", window.title.as_str(), entity);
            let sdl_window = context
                .windows
                .create_window(&context.sdl, entity, &*window)?;

            if let Some(theme) = match VideoSubsystem::get_system_theme() {
                sdl3::video::SystemTheme::Unknown => None,
                sdl3::video::SystemTheme::Light => Some(WindowTheme::Light),
                sdl3::video::SystemTheme::Dark => Some(WindowTheme::Dark),
            } {
                window.window_theme = Some(theme);
            }

            window
                .resolution
                .set_scale_factor(sdl_window.display_scale());

            // sdl_window.maximize()
            // sdl_window.minimize()
            // sdl_window.hide()
            // sdl_window.opacity()
            // sdl_window.raise()
            // sdl_window.restore()
            // sdl_window.set_mouse_rect(rect)
            // sdl_window.set_bordered(bordered)
            // sdl_window.set_display_mode(display_mode)
            // sdl_window.set_fullscreen(fullscreen)
            // sdl_window.set_icon(icon)
            // sdl_window.set_keyboard_grab(grabbed)
            // sdl_window.set_maximum_size(width, height)
            // sdl_window.set_minimum_size(width, height)
            // sdl_window.set_mouse_grab(grabbed)
            // sdl_window.set_opacity(opacity)
            // sdl_window.set_position(x, y)
            // sdl_window.set_size(width, height)
            // sdl_window.set_title(title)

            window
                .resolution
                .set_scale_factor_and_apply_to_physical_size(sdl_window.display_scale());

            if let Ok(handle_wrapper) = RawHandleWrapper::new(sdl_window) {
                commands.entity(entity).insert(handle_wrapper.clone());
                if let Some(handle_holder) = handle_holder {
                    *handle_holder.0.lock().unwrap() = Some(handle_wrapper);
                }
            }
        }

        Ok::<_, BevyError>(())
    })?;

    Ok(())
}

pub fn handle_window_events(
    world: &mut World,
    _timestamp: u64,
    window_id: u32,
    event: WindowEvent,
) {
    let (window_entity, window_scale) = SDL_CONTEXT
        .with_borrow(SdlContext::get_window_entity_and_scale(window_id))
        .unwrap();

    let Ok(mut entity_mut) = world.get_entity_mut(window_entity) else {
        return;
    };
    let mut bevy_window = entity_mut.get_mut::<Window>().unwrap();

    match event {
        WindowEvent::Shown => {
            world.send_event(WindowOccluded {
                window: window_entity,
                occluded: false,
            });
        }
        WindowEvent::Hidden => {
            world.send_event(WindowOccluded {
                window: window_entity,
                occluded: true,
            });
        }
        WindowEvent::Exposed => {
            world.send_event(WindowOccluded {
                window: window_entity,
                occluded: false,
            });
        }
        WindowEvent::Moved(x, y) => {
            let position = IVec2::new(x, y);
            bevy_window.position.set(position);
            world.send_event(WindowMoved {
                window: window_entity,
                position: IVec2::new(x, y),
            });
        }
        WindowEvent::Resized(width, height) => {
            bevy_window
                .resolution
                .set_physical_resolution(width as u32, height as u32);
            bevy_window.resolution.set_scale_factor(window_scale);

            world.send_event(WindowResized {
                window: window_entity,
                width: width as f32,
                height: height as f32,
            });
        }
        WindowEvent::PixelSizeChanged(width, height) => {
            bevy_window
                .resolution
                .set_physical_resolution(width as u32, height as u32);
            bevy_window.resolution.set_scale_factor(window_scale);

            world.send_event(WindowResized {
                window: window_entity,
                width: width as f32,
                height: height as f32,
            });
        }
        WindowEvent::MouseEnter => {
            world.send_event(CursorEntered {
                window: window_entity,
            });
        }
        WindowEvent::MouseLeave => {
            world.send_event(CursorLeft {
                window: window_entity,
            });
        }
        WindowEvent::FocusGained => {
            bevy_window.focused = true;

            world.send_event(WindowFocused {
                window: window_entity,
                focused: true,
            });
        }
        WindowEvent::FocusLost => {
            bevy_window.focused = false;

            world.send_event(WindowFocused {
                window: window_entity,
                focused: false,
            });
        }
        WindowEvent::CloseRequested => {
            world.send_event(WindowCloseRequested {
                window: window_entity,
            });
        }
        // TODO: check if window occluded and resized events are sent when these are
        WindowEvent::Minimized | WindowEvent::Maximized | WindowEvent::Restored => {}
        // WindowEvent::None => {}
        // WindowEvent::HitTest(_, _) => {}
        // WindowEvent::ICCProfChanged => {}
        // WindowEvent::DisplayChanged(_) => {}
        e => {
            dbg!(e);
        }
    }
}

// push changes to bevy window to sdl
pub fn update_windows() {}
