use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Weapon)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key : &Option<VirtualKeyCode>,
    #[resource] turn_state : &mut TurnState
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

    if let Some(key) = *key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::G => {
                let (player, player_pos) = players
                    .iter(ecs)
                    .find_map(|(entity, pos)| Some((*entity, *pos)))
                    .unwrap();

                let mut items = <(Entity, &Item, &Point)>::query();
                items.iter(ecs)
                    .filter(|(_entity, _item, &item_pos)| item_pos == player_pos)
                    .for_each(|(entity, _item, _item_pos)| {
                        commands.remove_component::<Point>(*entity);
                        commands.add_component(*entity, Carried(player));

                        if let Ok(e) = ecs.entry_ref(*entity) {
                            if e.get_component::<Weapon>().is_ok() {// (1)
                                <(Entity, &Carried, &Weapon)>::query()
                                .iter(ecs)
                                .filter(|(_, c, _)| c.0 == player)
                                .for_each(|(e, c, w)| {
                                    commands.remove(*e);// (2)
                                })
                            }
                        }
                    }
                );
                Point::new(0, 0)
            },
            VirtualKeyCode::Key1 => use_item(0, ecs, commands),
            VirtualKeyCode::Key2 => use_item(1, ecs, commands),
            VirtualKeyCode::Key3 => use_item(2, ecs, commands),
            VirtualKeyCode::Key4 => use_item(3, ecs, commands),
            VirtualKeyCode::Key5 => use_item(4, ecs, commands),
            VirtualKeyCode::Key6 => use_item(5, ecs, commands),
            VirtualKeyCode::Key7 => use_item(6, ecs, commands),
            VirtualKeyCode::Key8 => use_item(7, ecs, commands),
            VirtualKeyCode::Key9 => use_item(8, ecs, commands),
            _ => Point::new(0, 0),
        };

        let (player_entity, destination) = players
                .iter(ecs)
                .find_map(|(entity, pos)| Some((*entity, *pos + delta)) )
                .unwrap();

        let mut did_something = false;
        if delta.x !=0 || delta.y != 0 {

        let mut hit_something = false;
        enemies
            .iter(ecs)
            .filter(|(_, pos)| {
                **pos == destination
            })
            .for_each(|(entity, _) | {
                hit_something = true;
                did_something = true;

                commands
                    .push(((), WantsToAttack{
                        attacker: player_entity,
                        victim: *entity,
                    }));
            });

            if !hit_something {
                did_something = true;
                commands
                    .push(((), WantsToMove{
                        entity: player_entity,
                        destination
                    }));
            }
        };
        *turn_state = TurnState::PlayerTurn;
    }
}

fn use_item(n: usize, ecs: &mut SubWorld, commands: &mut CommandBuffer) 
-> Point {
    let player_entity = <(Entity, &Player)>::query()
                    .iter(ecs)
                    .find_map(|(entity, _player)| Some(*entity))
                    .unwrap();

    let item_entity = <(Entity, &Item, &Carried)>::query()
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player_entity)
        .enumerate()
        .filter(|(item_count, (_, _, _))| *item_count == n)
        .find_map(|(_, (item_entity, _, _))| Some(*item_entity));

    if let Some(item_entity) = item_entity {
        commands
            .push(((), ActivateItem{
                used_by: player_entity,
                item: item_entity
            }));
    }

    Point::zero()
}
