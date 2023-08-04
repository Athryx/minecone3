use bevy::prelude::*;

const CROSSHAIR_SIZE: Val = Val::Px(30.0);

fn init_crosshair(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn(ImageBundle {
            style: Style {
                display: Display::Flex,
                align_self: AlignSelf::Center,
                min_width: CROSSHAIR_SIZE,
                max_width: CROSSHAIR_SIZE,
                min_height: CROSSHAIR_SIZE,
                max_height: CROSSHAIR_SIZE,
                ..Default::default()
            },
            image: UiImage {
                texture: asset_server.load("textures/ui/crosshair.png"),
                ..Default::default()
            },
            ..Default::default()
        });
    });
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_crosshair);
    }
}