mod utils;
mod wiki_node;
use bevy::prelude::*;
use bevy::window::{WindowDescriptor, WindowPlugin, WindowResized};
use wasm_bindgen::prelude::*;
use wiki_node::{add_nodes, sprite_position_update, step_nodes};

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
pub fn bevy(canvas: &str) {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                canvas: Some(canvas.to_string()),
                fit_canvas_to_parent: true,
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
