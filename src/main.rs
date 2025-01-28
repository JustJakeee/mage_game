use bevy::prelude::*;
use rand::random;

#[derive(Default, Component)]
struct Bounds(Rect);
#[derive(Default, Component)]
struct Position(Vec2);
#[derive(Default, Component)]
struct Velocity(Vec2);
#[derive(Component)]
struct MaxSpeed(f32);
#[derive(Component)]
struct Acceleration(f32);
#[derive(Component)]
struct Player;
#[derive(Component)]
struct Shaking(f32);
#[derive(Resource)]
struct Textures {
    player_front: Handle<Image>,
    player_back: Handle<Image>,
    scythe: Handle<Image>,
}
#[derive(Resource)]
struct FrictionAcceleration(f32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(1.0, 0.9, 0.9)))
        .insert_resource(FrictionAcceleration(5000.))
        .add_systems(Startup, (load, setup.after(load)))
        .add_systems(
            Update,
            (
                update_transform_with_position,
                shake_transform_with_position,
                update_position_with_velocity,
                update_player_velocity_with_input,
                update_velocity_with_friction,
            ),
        )
        .run();
}

fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
    let textures = Textures {
        player_front: asset_server.load("sprites/guy_front.png"),
        player_back: asset_server.load("sprites/guy_back.png"),
        scythe: asset_server.load("sprites/scythe.png"),
    };
    commands.insert_resource(textures);
}

fn setup(mut commands: Commands, textures: Res<Textures>) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scale: 2.,
            ..OrthographicProjection::default_2d()
        },
    ));

    commands.spawn((
        Sprite::from_image(textures.player_front.clone()),
        Position(Vec2::new(300., 0.)),
        Velocity(Vec2::new(0., 0.)),
        Player,
        MaxSpeed(500.),
        Acceleration(10000.),
    ));

    commands.spawn((
        Sprite::from_image(textures.player_front.clone()),
        Position(Vec2::new(-300., 0.)),
        Shaking(10.),
    ));
}

fn update_transform_with_position(mut query: Query<(&mut Transform, &Position), Without<Shaking>>) {
    for (mut transform, position) in query.iter_mut() {
        transform.translation = position.0.extend(0.);
    }
}

fn shake_transform_with_position(mut query: Query<(&mut Transform, &Position, &Shaking)>) {
    for (mut transform, position, shaking) in query.iter_mut() {
        transform.translation = (position.0
            + Vec2::new(random::<f32>() * shaking.0, random::<f32>() * shaking.0))
        .extend(0.);
    }
}

fn update_position_with_velocity(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
    for (mut position, velocity) in query.iter_mut() {
        position.0 += velocity.0 * time.delta_secs()
    }
}

fn update_player_velocity_with_input(
    mut query: Query<(&mut Velocity, &MaxSpeed, &Acceleration), With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut velocity, max_speed, acceleration) in query.iter_mut() {
        let vertical_direction = (input.pressed(KeyCode::KeyW) as u8 as f32)
            - (input.pressed(KeyCode::KeyS) as u8 as f32);
        let horizontal_direction = (input.pressed(KeyCode::KeyD) as u8 as f32)
            - (input.pressed(KeyCode::KeyA) as u8 as f32);
        let normalized_direction =
            Vec2::new(horizontal_direction, vertical_direction).normalize_or_zero();
        let new_velocity = velocity.0 + (normalized_direction * acceleration.0 * time.delta_secs());
        if velocity.0.length() > max_speed.0 {
            return;
        }
        if new_velocity.length() < max_speed.0 {
            velocity.0 = new_velocity
        } else {
            velocity.0 = new_velocity.normalize() * max_speed.0
        }
    }
}

fn update_velocity_with_friction(
    mut query: Query<&mut Velocity>,
    friction_acceleration: Res<FrictionAcceleration>,
    time: Res<Time>,
) {
    for mut velocity in query.iter_mut() {
        let previous_velocity = velocity.0;
        if velocity.0.length() > friction_acceleration.0 * 2. * time.delta_secs() {
            velocity.0 -=
                previous_velocity.normalize_or_zero() * friction_acceleration.0 * time.delta_secs();
        } else {
            velocity.0 = Vec2::ZERO;
        }
    }
}
