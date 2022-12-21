use bevy::prelude::*;
use bevy_aspect_ratio_camera::*;

#[derive(Component, Clone, Copy)]
struct MainArea;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(BorderAspectRatioPlugin {
            color: Color::BLACK,
            marker: MainArea,
        })
        .add_plugin(CameraAspectRatioPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        AspectCamera {
            width: 400.,
            height: 200.,
        },
    ));

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(32., 32.)),
            color: Color::RED,
            ..Default::default()
        },
        ..Default::default()
    });
}
