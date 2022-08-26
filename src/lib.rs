mod audio;
mod loading;
mod menu;
mod new_game;
mod player;

use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

use bevy::app::App;
// #[cfg(debug_assertions)]
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;
use leafwing_input_manager::{prelude::InputManagerPlugin, Actionlike};
use new_game::NewGamePlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    LoadingAssets,
    Menu,
    NewGame,
    Playing,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(GameState::LoadingAssets)
            .add_plugin(InputManagerPlugin::<Action>::default())
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(NewGamePlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin);

        // Add back in for performance info
        // #[cfg(debug_assertions)]
        // {
        //     app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        //         .add_plugin(LogDiagnosticsPlugin::default());
        // }
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    Move,
    Jump,
}
