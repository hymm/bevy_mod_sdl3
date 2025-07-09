use std::{collections::HashMap, error::Error};

use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    entity::{Entity, EntityHashMap},
    error::BevyError,
    system::{Commands, Query},
    world::World,
};
use bevy_window::{RawHandleWrapper, RawHandleWrapperHolder, Window, WindowResized, WindowWrapper};
use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle,
};
use sdl3::{Sdl, event::WindowEvent, video::Window as Sdl3Window};
use tracing::info;

use crate::{SDL_CONTEXT, non_send_marker::NonSendMarker};

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
        _window: &Window,
    ) -> Result<&WindowWrapper<SyncWindow>, Box<dyn Error + Send + Sync>> {
        let video = sdl.video()?;
        let window = video
            .window("Window Name", 800, 600)
            .position_centered()
            .resizable()
            .metal_view()
            .build()
            .map_err(|e| e.to_string())?;
        let id = WindowId(window.id());
        self.windows
            .insert(id, WindowWrapper::new(SyncWindow(window)));
        self.entity_to_winit.insert(entity, id);
        self.winit_to_entity.insert(id, entity);

        Ok(self.windows.get(&id).unwrap())
    }

    fn get_window(&self, entity: Entity) -> Option<&WindowWrapper<SyncWindow>> {
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

            window
                .resolution
                .set_scale_factor_and_apply_to_physical_size(sdl_window.display_scale());

            // cache the window to detect changes
            // commands.entity(entity).insert((
            //     CachedWindow(window.clone()),
            //     CachedCursorOptions(cursor_options.clone()),
            //     WinitWindowPressedKeys::default(),
            // ));

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

pub fn handle_window_events(world: &mut World, timestamp: u64, window_id: u32, event: WindowEvent) {
    match event {
        WindowEvent::None => todo!(),
        WindowEvent::Shown => todo!(),
        WindowEvent::Hidden => todo!(),
        WindowEvent::Exposed => todo!(),
        WindowEvent::Moved(_, _) => todo!(),
        WindowEvent::Resized(width, heights) => {
            let window = SDL_CONTEXT
                .with_borrow(|context| context.as_mut().unwrap().windows.get_window(entity));
            window.resolution.set_physical_resolution(width, height);

            world.write(WindowResized {
                window: window_entity,
                width: window.width(),
                height: window.height(),
            });
        }
        WindowEvent::PixelSizeChanged(_, _) => todo!(),
        WindowEvent::Minimized => todo!(),
        WindowEvent::Maximized => todo!(),
        WindowEvent::Restored => todo!(),
        WindowEvent::MouseEnter => todo!(),
        WindowEvent::MouseLeave => todo!(),
        WindowEvent::FocusGained => todo!(),
        WindowEvent::FocusLost => todo!(),
        WindowEvent::CloseRequested => todo!(),
        WindowEvent::HitTest(_, _) => todo!(),
        WindowEvent::ICCProfChanged => todo!(),
        WindowEvent::DisplayChanged(_) => todo!(),
    }
}
