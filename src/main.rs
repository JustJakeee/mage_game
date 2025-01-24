use bevy::prelude::*;

#[derive(Default, Component)]
struct Bounds(Rect);
#[derive(Default, Component)]
struct Velocity(Vec2);
#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scale: 2.,
            ..OrthographicProjection::default_2d()
        },
    ));

    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/guy_front.png")),
        Player,
    ));
}

fn update(mut query: Query<&mut Transform, With<Player>>, time: Res<Time>) {
    let mut transform = query.single_mut();
    transform.translation.x += time.delta_secs() * 30.;
}
