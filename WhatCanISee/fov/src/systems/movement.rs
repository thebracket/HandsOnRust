use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]// (1)
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer
) {
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
}
