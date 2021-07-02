use crate::prelude::*;

#[system]
pub fn end_turn(#[resource] turn_state: &mut TurnState) {// (1)
    let new_state = match turn_state {
        TurnState::AwaitingInput => return,// (2)
        TurnState::PlayerTurn => TurnState::MonsterTurn,// (3)
        TurnState::MonsterTurn => TurnState::AwaitingInput// (4)
    };

    *turn_state = new_state;// (5)
}
