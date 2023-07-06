use bevy::prelude::*;

use minecone::MineconePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Minecone".into(),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(AssetPlugin {
                asset_folder: String::from("/home/jack/projects/games/minecone3/assets"),
                watch_for_changes: false,
            })
        )
        .add_plugin(MineconePlugin)
        .run();
}
