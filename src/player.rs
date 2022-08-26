use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};
use leafwing_input_manager::prelude::InputManagerPlugin;
use leafwing_input_manager::{errors::NearlySingularConversion, orientation::Direction};
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    Actionlike, InputManagerBundle,
};

pub struct PlayerPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerMovement>()
            .add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .add_enter_system(GameState::Playing, spawn_player)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Playing)
                    .with_system(control_player_movement)
                    .with_system(move_player)
                    .into(),
            );
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum PlayerAction {
    // Movement
    Up,
    Down,
    Left,
    Right,
    // Abilities
    Ability1,
    Ability2,
    Ability3,
    Ability4,
    Ultimate,
}

impl PlayerAction {
    // Lists like this can be very useful for quickly matching subsets of actions
    const DIRECTIONS: [Self; 4] = [
        PlayerAction::Up,
        PlayerAction::Down,
        PlayerAction::Left,
        PlayerAction::Right,
    ];

    fn direction(self) -> Option<Direction> {
        match self {
            PlayerAction::Up => Some(Direction::NORTH),
            PlayerAction::Down => Some(Direction::SOUTH),
            PlayerAction::Right => Some(Direction::EAST),
            PlayerAction::Left => Some(Direction::WEST),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct PlayerMovement {
    pub direction: Direction,
}

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    // This bundle must be added to your player entity
    // (or whatever else you wish to control)
    #[bundle]
    input_manager: InputManagerBundle<PlayerAction>,
    #[bundle]
    sprite: SpriteBundle,
}

impl PlayerBundle {
    fn default_input_map() -> InputMap<PlayerAction> {
        // This allows us to replace `ArpgAction::Up` with `Up`,
        // significantly reducing boilerplate
        use PlayerAction::*;
        let mut input_map = InputMap::default();

        // Movement
        input_map.insert(KeyCode::Up, Up);
        input_map.insert(GamepadButtonType::DPadUp, Up);

        input_map.insert(KeyCode::Down, Down);
        input_map.insert(GamepadButtonType::DPadDown, Down);

        input_map.insert(KeyCode::Left, Left);
        input_map.insert(GamepadButtonType::DPadLeft, Left);

        input_map.insert(KeyCode::Right, Right);
        input_map.insert(GamepadButtonType::DPadRight, Right);

        // Abilities
        input_map.insert(KeyCode::Q, Ability1);
        input_map.insert(GamepadButtonType::West, Ability1);
        input_map.insert(MouseButton::Left, Ability1);

        input_map.insert(KeyCode::W, Ability2);
        input_map.insert(GamepadButtonType::North, Ability2);
        input_map.insert(MouseButton::Right, Ability2);

        input_map.insert(KeyCode::E, Ability3);
        input_map.insert(GamepadButtonType::East, Ability3);

        input_map.insert(KeyCode::Space, Ability4);
        input_map.insert(GamepadButtonType::South, Ability4);

        input_map.insert(KeyCode::R, Ultimate);
        input_map.insert(GamepadButtonType::LeftTrigger2, Ultimate);

        input_map
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn_bundle(PlayerBundle {
        player: Player,
        input_manager: InputManagerBundle {
            input_map: PlayerBundle::default_input_map(),
            action_state: ActionState::default(),
        },
        sprite: SpriteBundle {
            texture: textures.texture_bevy.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        },
    });
}

fn move_player(
    mut events: EventReader<PlayerMovement>,
    mut q_player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    for mut player_transform in &mut q_player {
        for event in events.iter() {
            player_transform.translation +=
                (event.direction * 150.0 * time.delta_seconds()).extend(0.0);
        }
    }
    for my_event in events.iter() {
        info!("{}", my_event.direction);
    }
}

fn control_player_movement(
    query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut event_writer: EventWriter<PlayerMovement>,
) {
    let action_state = query.single();

    let mut direction_vector = Vec2::ZERO;

    for input_direction in PlayerAction::DIRECTIONS {
        if action_state.pressed(input_direction) {
            if let Some(direction) = input_direction.direction() {
                // Sum the directions as 2D vectors
                direction_vector += Vec2::from(direction);
            }
        }
    }

    // Then reconvert at the end, normalizing the magnitude
    let net_direction: Result<Direction, NearlySingularConversion> = direction_vector.try_into();

    if let Ok(direction) = net_direction {
        event_writer.send(PlayerMovement { direction });
    }
}
