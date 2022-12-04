mod hash_grid;
mod utils;
mod wiki_node;
use bevy::prelude::*;
use bevy::window::{WindowDescriptor, WindowPlugin};
use bevy_diagnostic::{
    DiagnosticsPlugin, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
    LogDiagnosticsPlugin,
};
use hash_grid::HashGrid;
use wasm_bindgen::prelude::*;
use wiki_node::{node_repulsion, spawn_nodes, sprite_position_update, step_nodes};

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

// 1500 / 60fps

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
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(EntityCountDiagnosticsPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_startup_system(setup)
        .insert_resource(HashGrid { grid_size: 32.0 })
        .add_system(step_nodes)
        .add_system(spawn_nodes)
        .add_system(node_repulsion)
        .add_system(sprite_position_update)
        .run();
}

fn setup(mut commands: Commands, mut windows: ResMut<Windows>) {
    windows
        .get_primary_mut()
        .unwrap()
        .update_scale_factor_from_backend(1.0);
    commands.spawn(Camera2dBundle::default());
}
