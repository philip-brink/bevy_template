use bevy::prelude::{App, Assets, Commands, EventReader, Handle, Plugin, Res, ResMut};
use bevy_kira_audio::{Audio, AudioControl, AudioInstance, AudioPlugin, AudioTween, PlaybackState};
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};

use crate::loading::AudioAssets;
use crate::player::PlayerMovement;
use crate::GameState;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_enter_system(GameState::Playing, start_audio)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Playing)
                    .with_system(control_flying_sound)
                    .into(),
            );
    }
}

struct FlyingAudio(Handle<AudioInstance>);

fn start_audio(mut commands: Commands, audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.pause();
    let handle = audio
        .play(audio_assets.flying.clone())
        .looped()
        .with_volume(0.3)
        .handle();
    commands.insert_resource(FlyingAudio(handle));
}

fn control_flying_sound(
    player_walk_events: EventReader<PlayerMovement>,
    audio: Res<FlyingAudio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Some(instance) = audio_instances.get_mut(&audio.0) {
        match instance.state() {
            PlaybackState::Paused { .. } => {
                if !player_walk_events.is_empty() {
                    instance.resume(AudioTween::default());
                }
            }
            PlaybackState::Playing { .. } => {
                if player_walk_events.is_empty() {
                    instance.pause(AudioTween::default());
                }
            }
            _ => {}
        }
    }
}
