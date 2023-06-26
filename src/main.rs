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
        )
        .add_plugin(MineconePlugin)
        .run();
}
