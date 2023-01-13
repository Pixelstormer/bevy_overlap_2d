use bevy::prelude::*;
use bevy_overlap_2d::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Speed(f32);

#[derive(Component, Default)]
struct Velocity(Vec2);

#[derive(Component)]
struct Friction(f32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CollisionPlugin)
        .add_startup_system(spawn_world)
        .add_system(player_movement)
        .add_system(move_objects.after(player_movement))
        .add_system_to_stage(
            CollisionStage,
            collide_player_ball.after(FindCollidingPairs),
        )
        .run()
}

fn spawn_world(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // The player
    commands.spawn((
        ColliderBundle::capsule(15.0, 15.0),
        Speed(5.0),
        Player,
        Velocity::default(),
        ColliderDrawBundle::default(),
    ));

    // The pushable balls
    commands
        .spawn((
            ColliderBundle::circle(15.0),
            Ball,
            Velocity::default(),
            Friction(0.75),
            ColliderDrawBundle::default(),
        ))
        .insert(Transform::from_xyz(100.0, 50.0, 0.0));

    commands
        .spawn((
            ColliderBundle::circle(15.0),
            Ball,
            Velocity::default(),
            Friction(0.75),
            ColliderDrawBundle::default(),
        ))
        .insert(Transform::from_xyz(100.0, -50.0, 0.0));

    // The four walls
    commands
        .spawn((
            ColliderBundle::rect(Rect::from_center_half_size(
                Vec2::ZERO,
                Vec2::new(25.0, 100.0),
            )),
            Wall,
            ColliderDrawBundle::default(),
        ))
        .insert(Transform::from_xyz(250.0, 0.0, 0.0));

    commands
        .spawn((
            ColliderBundle::rect(Rect::from_center_half_size(
                Vec2::ZERO,
                Vec2::new(25.0, 100.0),
            )),
            Wall,
            ColliderDrawBundle::default(),
        ))
        .insert(Transform::from_xyz(-250.0, 0.0, 0.0));

    commands
        .spawn((
            ColliderBundle::rect(Rect::from_center_half_size(
                Vec2::ZERO,
                Vec2::new(100.0, 25.0),
            )),
            Wall,
            ColliderDrawBundle::default(),
        ))
        .insert(Transform::from_xyz(0.0, 250.0, 0.0));

    commands
        .spawn((
            ColliderBundle::rect(Rect::from_center_half_size(
                Vec2::ZERO,
                Vec2::new(100.0, 25.0),
            )),
            Wall,
            ColliderDrawBundle::default(),
        ))
        .insert(Transform::from_xyz(0.0, -250.0, 0.0));
}

fn player_movement(mut query: Query<(&mut Velocity, &Speed)>, input: Res<Input<KeyCode>>) {
    let mut target_velocity = Vec2::ZERO;

    if input.any_pressed([KeyCode::W, KeyCode::Up]) {
        target_velocity.y += 1.0;
    }

    if input.any_pressed([KeyCode::A, KeyCode::Left]) {
        target_velocity.x -= 1.0;
    }

    if input.any_pressed([KeyCode::S, KeyCode::Down]) {
        target_velocity.y -= 1.0;
    }

    if input.any_pressed([KeyCode::D, KeyCode::Right]) {
        target_velocity.x += 1.0;
    }

    for (mut velocity, speed) in query.iter_mut() {
        velocity.0 = velocity.0.lerp(target_velocity * speed.0, 0.2);
    }
}

fn move_objects(mut query: Query<(&mut Transform, &mut Velocity, Option<&Friction>)>) {
    for (mut transform, mut velocity, friction) in query.iter_mut() {
        if let Some(friction) = friction {
            velocity.0 *= friction.0;
        }

        transform.translation += velocity.0.extend(0.0);
    }
}

fn collide_player_ball(
    player_query: Query<(&Transform, &Velocity), (With<Player>, Without<Ball>)>,
    mut ball_query: Query<(&mut Transform, &mut Velocity), (With<Ball>, Without<Player>)>,
    mut collisions: EventReader<CollisionEvent>,
) {
    for collision in collisions.iter() {
        let CollisionEvent::Began(CollisionBegan { us, them , contact}) = collision else { continue; };

        let ContactManifold::Point(contact) = contact else { unreachable!() };

        let mut contact = contact;
        let player;
        let ball;

        if let Ok(p) = player_query.get(*us) {
            let Ok(b) = ball_query.get_mut(*them) else { continue; };
            player = p;
            ball = b;
        } else if let Ok(p) = player_query.get(*them) {
            let Ok(b) = ball_query.get_mut(*us) else { continue; };
            player = p;
            ball = b;
            contact.negate();
        } else {
            continue;
        }

        let (player_transform, player_velocity) = player;
        let (mut ball_transform, mut ball_velocity) = ball;

        let deintersect_vector = contact.separation_vector();
        ball_transform.translation += deintersect_vector;
        ball_velocity += deintersect_vector;
    }
}

fn collide_player_static(player: Entity, mut collisions: EventReader<CollisionEvent>) {}

fn collide_ball_ball(a: Entity, b: Entity, mut collisions: EventReader<CollisionEvent>) {}

fn collide_ball_static(ball: Entity, mut collisions: EventReader<CollisionEvent>) {}
