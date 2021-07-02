use crate::prelude::*;

#[system]
#[read_component(WantsToMove)]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(FieldOfView)]// (1)
pub fn movement(
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &SubWorld,
    commands: &mut CommandBuffer
) {
    let mut movers = <(Entity, &WantsToMove)>::query();
    movers.iter(ecs).for_each(| (entity, want_move) | {
        if map.can_enter_tile(want_move.destination) {
            commands.add_component(want_move.entity, want_move.destination);

            if let Ok(entry) = ecs.entry_ref(want_move.entity) {// (2)
                if let Ok(fov) = entry.get_component::<FieldOfView>() {
                    commands.add_component(want_move.entity, fov.clone_dirty());// (3)
                }

                if entry.get_component::<Player>().is_ok()
                {
                    camera.on_player_move(want_move.destination);
                }
            }
        }
        commands.remove(*entity);
    });
}