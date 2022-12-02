mod utils;
mod wiki_node;
use bevy::prelude::*;
use bevy::window::WindowDescriptor;
use bevy::window::WindowPlugin;
use wasm_bindgen::prelude::*;
use wiki_node::add_nodes;
use wiki_node::sprite_position_update;
use wiki_node::step_nodes;
use wiki_node::PhysicsObject;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, bevy-nodes!");
}

#[wasm_bindgen]
pub fn bevy() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                canvas: Some("#nodes".to_string()),
                width: 640.0,
                height: 400.0,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(step_nodes)
        .add_system(sprite_position_update)
        .add_system(add_nodes)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
}
