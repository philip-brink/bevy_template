use bevy::prelude::*;

use crate::GameState;

pub struct AquariumPlugin;

impl Plugin for AquariumPlugin {
    fn build(&self, app: &mut App) {
        // app.init_resource::<Aquarium>();
        app.add_system_set(SystemSet::on_enter(GameState::Setup).with_system(setup))
            .add_system_set(SystemSet::on_update(GameState::Setup).with_system(finish_setup))
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(fluctuate_water));
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(SpatialBundle::default())
        .insert(Aquarium)
        .with_children(|parent| {
            parent.spawn_bundle(WallBundle::new(WallLocation::Left));
            parent.spawn_bundle(WallBundle::new(WallLocation::Bottom));
            parent.spawn_bundle(WallBundle::new(WallLocation::Right));
            parent.spawn_bundle(WaterBundle::new());
        });
}

fn finish_setup(mut state: ResMut<State<GameState>>) {
    state.set(GameState::Playing).unwrap();
}

fn fluctuate_water(mut q_water: Query<(&mut Water, &mut Transform)>, time: Res<Time>) {
    let low_water_goal = 0.9;
    let high_water_goal = 0.95;
    let speed = 0.01;
    for (mut water, mut transform) in &mut q_water {
        match water.direction {
            WaterDirection::Filling => {
                water.level += time.delta_seconds() * speed;
                if water.level >= water.goal_level {
                    water.goal_level = low_water_goal;
                    water.direction = WaterDirection::Emptying;
                }
            }
            WaterDirection::Emptying => {
                water.level -= time.delta_seconds() * speed;
                if water.level <= water.goal_level {
                    water.goal_level = high_water_goal;
                    water.direction = WaterDirection::Filling;
                }
            }
        }

        transform.translation = water.water_translation().extend(0.0);
        transform.scale = water.water_size().extend(1.0);
    }
}

const WALL_THICKNESS: f32 = 2.0;
const LEFT_WALL: f32 = -250.;
const RIGHT_WALL: f32 = 250.;
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;
const WALL_COLOR: Color = Color::rgb(0.5, 0.7, 0.8);
const WATER_COLOR: Color = Color::rgb(0.6, 0.8, 0.9);

#[derive(Bundle)]
pub struct WaterBundle {
    pub water: Water,
    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl WaterBundle {
    fn new() -> WaterBundle {
        let water = Water::new(0.95);
        WaterBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: water.water_translation().extend(0.0),
                    scale: water.water_size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WATER_COLOR,
                    ..default()
                },
                ..default()
            },
            water,
        }
    }
}

#[derive(Component)]
pub struct Water {
    level: f32,
    direction: WaterDirection,
    goal_level: f32,
}

impl Water {
    pub fn surface(&self) -> f32 {
        (self.water_size().y / 2.0) + self.water_translation().y
    }

    pub fn bottom(&self) -> f32 {
        self.water_translation().y - (self.water_size().y / 2.0)
    }

    fn new(level: f32) -> Self {
        assert!((0.0..=1.0).contains(&level));
        Water {
            level,
            direction: WaterDirection::Emptying,
            goal_level: level,
        }
    }

    fn water_translation(&self) -> Vec2 {
        let height = TOP_WALL - BOTTOM_WALL - WALL_THICKNESS;
        let vertical_offset = height * (1.0 - self.level) / -2.0;
        Vec2::new(0.0, vertical_offset)
    }

    fn water_size(&self) -> Vec2 {
        let height = (TOP_WALL - BOTTOM_WALL - WALL_THICKNESS) * self.level;
        let width = RIGHT_WALL - LEFT_WALL - WALL_THICKNESS;
        Vec2::new(width, height)
    }
}

enum WaterDirection {
    Filling,
    Emptying,
}

#[derive(Component)]
struct Collider;

#[derive(Default)]
struct CollisionEvent;

#[derive(Component)]
struct Aquarium;

#[derive(Bundle)]
struct WallBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

impl WallBundle {
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position().extend(0.0),
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let height = TOP_WALL - BOTTOM_WALL;
        let width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(height > 0.0);
        assert!(width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}
