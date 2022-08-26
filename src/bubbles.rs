use crate::{aquarium::Water, loading::TextureAssets, GameState};
use bevy::prelude::*;
use std::time::Duration;

pub struct BubblesPlugin;

impl Plugin for BubblesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_timer))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(spawn_bubble))
            .add_system(move_bubbles)
            .add_system(despawn_bubbles);
    }
}

#[derive(Component)]
struct BubbleSpawner {
    timer: Timer,
}

#[derive(Component)]
struct Bubble;

fn setup_timer(mut commands: Commands) {
    commands.spawn().insert(BubbleSpawner {
        timer: Timer::new(Duration::from_millis(500), true),
    });
}
fn spawn_bubble(
    mut commands: Commands,
    mut q_bubble_spawner: Query<&mut BubbleSpawner>,
    q_water: Query<&Water>,
    textures: Res<TextureAssets>,
    time: Res<Time>,
) {
    if q_water.is_empty() {
        return;
    }
    let water = q_water.single();

    for mut bubble_spawner in &mut q_bubble_spawner {
        bubble_spawner.timer.tick(time.delta());
        if bubble_spawner.timer.finished() {
            commands
                .spawn_bundle(SpriteBundle {
                    texture: textures.bubble.clone(),
                    transform: Transform {
                        translation: Vec3::new(0.0, water.bottom(), 1.0),
                        scale: Vec3::new(0.1, 0.1, 1.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(Bubble);
        }
    }
}

fn move_bubbles(mut q_bubbles: Query<&mut Transform, With<Bubble>>, time: Res<Time>) {
    for mut bubble_transform in &mut q_bubbles {
        bubble_transform.translation += Vec3::new(0.0, 100.0, 0.0) * time.delta_seconds();
    }
}

fn despawn_bubbles(
    mut commands: Commands,
    q_bubbles: Query<(Entity, &Transform), With<Bubble>>,
    q_water: Query<&Water>,
) {
    if q_water.is_empty() {
        return;
    }
    let water = q_water.single();

    for (entity, transform) in &q_bubbles {
        if transform.translation.y > water.surface() {
            commands.entity(entity).despawn();
        }
    }
}
