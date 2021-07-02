use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn end_turn(
    ecs: &SubWorld,
    #[resource] turn_state: &mut TurnState
) {
    let mut player_hp = <&Health>::query().filter(component::<Player>());// (1)
    let current_state = turn_state.clone();
    let mut new_state = match current_state {// (2)
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state // (3)
    };

    player_hp.iter(ecs).for_each(|hp| {// (4)
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }
    });

    *turn_state = new_state;
}
