#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    GameOver,
    Victory,
    NextLevel
}