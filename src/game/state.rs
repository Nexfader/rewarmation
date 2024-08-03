use bevy::prelude::*;

#[derive(Clone, Copy, Debug, States, PartialEq, Eq, Hash)]
pub enum PlayingState {
    Playing,
    Paused,
}

#[derive(Clone, Copy, Debug, States, PartialEq, Eq, Hash)]
pub enum GamePlayState {
    InGame,
}

pub enum GameEndKind {
    TeamVictory,
    RoundDraw,
}

pub enum GameState {
    Start,
    SpawnPlayers,
    PlayerStartTurn,
    PlayerTurn,
    PlayerEndTurn,
    Spectation,
    End { kind: GameEndKind },
}
