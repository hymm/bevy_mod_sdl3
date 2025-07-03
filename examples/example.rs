use bevy::{
    prelude::{App, DefaultPlugins, PluginGroup},
    winit::WinitPlugin,
};
use bevy_mod_sdl3::Sdl3Plugin;

fn main() {
    App::default()
        .add_plugins((DefaultPlugins.build().disable::<WinitPlugin>(), Sdl3Plugin))
        .run();
}
