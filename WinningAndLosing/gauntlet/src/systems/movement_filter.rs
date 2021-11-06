use crate::prelude::*;

#[system]
#[read_component(WantsToMove)]
pub fn movement_filter(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer) {

    let mut moves = <(Entity, &WantsToMove)>::query();
    let mut pos_seen: Vec<Point> = Vec::new();

    moves.iter(ecs).for_each(|(entity, want_move)| {
        if pos_seen.contains(&want_move.destination) {
            commands.remove(*entity);
        }
        else {
            pos_seen.push(want_move.destination);
        }
    });
}