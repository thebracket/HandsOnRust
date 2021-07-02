use crate::prelude::*;

#[system]
#[read_component(Point)] // (1)
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {// (2)
    let mut player_pos = Point::zero();
    let mut players = <&Point>::query()
        .filter(component::<Player>());
    players.iter(ecs).for_each(|pos| player_pos = *pos);
    let mut enemies = <(Entity, &Point)>::query()
        .filter(component::<Enemy>());
    enemies
        .iter(ecs)
        .filter(|(_,pos)| **pos == player_pos)// (3)
        .for_each(|(entity, _)| {// (4)
            commands.remove(*entity);// (5)
        }
    );
}
