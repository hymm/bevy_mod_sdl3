use std::marker::PhantomData;

use bevy_ecs::{
    component::Tick,
    system::{ReadOnlySystemParam, SystemMeta, SystemParam},
    world::{World, unsafe_world_cell::UnsafeWorldCell},
};

/// A dummy type that is [`!Send`](Send), to force systems to run on the main thread.
pub struct NonSendMarker(PhantomData<*mut ()>);

// SAFETY: No world access.
unsafe impl SystemParam for NonSendMarker {
    type State = ();
    type Item<'w, 's> = Self;

    #[inline]
    fn init_state(_world: &mut World, system_meta: &mut SystemMeta) -> Self::State {
        system_meta.set_non_send();
    }

    #[inline]
    unsafe fn get_param<'world, 'state>(
        _state: &'state mut Self::State,
        _system_meta: &SystemMeta,
        _world: UnsafeWorldCell<'world>,
        _change_tick: Tick,
    ) -> Self::Item<'world, 'state> {
        Self(PhantomData)
    }
}

// SAFETY: Does not read any world state
unsafe impl ReadOnlySystemParam for NonSendMarker {}
