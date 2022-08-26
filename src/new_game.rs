use crate::GameState;
use bevy::prelude::*;
use iyes_loopless::{prelude::ConditionSet, state::NextState};

pub struct NewGamePlugin;

impl Plugin for NewGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::NewGame)
                .with_system(start_game)
                .into(),
        );
    }
}

fn start_game(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::Playing));
}
