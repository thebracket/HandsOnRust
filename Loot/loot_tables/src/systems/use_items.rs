use crate::prelude::*;

#[system]
#[read_component(ActivateItem)]
#[read_component(ProvidesHealing)]
#[write_component(Health)]
#[read_component(ProvidesDungeonMap)]
pub fn use_items(
    ecs: &mut SubWorld, 
    commands: &mut CommandBuffer,
    #[resource] map: &mut Map
) {
    let mut healing_to_apply = Vec::<(Entity, i32)>::new();
    <(Entity, &ActivateItem)>::query().iter(ecs).for_each(|(entity, activate)| {// (1)

        let item = ecs.entry_ref(activate.item);// (2)
        if let Ok(item) = item {// (3)
            if let Ok(healing) = item.get_component::<ProvidesHealing>() {// (4)
                healing_to_apply.push((activate.used_by, healing.amount));// (5)
            }

            if let Ok(_mapper) = item.get_component::<ProvidesDungeonMap>() {// (6)
                map.revealed_tiles.iter_mut().for_each(|t| *t = true);// (7)
            }
        }

        commands.remove(activate.item);// (8)
        commands.remove(*entity);// (9)
    });

    for heal in healing_to_apply.iter() {
        if let Ok(mut target) = ecs.entry_mut(heal.0) {// (10)
            if let Ok(health) = target.get_component_mut::<Health>() {// (11)
                health.current = i32::min(health.max, health.current + heal.1);// (12)
            }
        }
    }
}
