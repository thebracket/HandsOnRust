use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Name)]
pub fn hud(ecs: &SubWorld) {
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let player_health = health_query
        .iter(ecs)
        .nth(0)
        .unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    draw_batch.print_centered(1, 
        "Explore the Dungeon. Cursor keys to move.");
    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH*2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK)
    );
    draw_batch.print_color_centered(
        0,
        format!(" Health: {} / {} ", 
            player_health.current, 
            player_health.max
        ),
        ColorPair::new(WHITE, RED)
    );

    let player = <(Entity, &Player)>::query()
                    .iter(ecs)
                    .find_map(|(entity, _player)| Some(*entity))
                    .unwrap();// (1)
    let mut item_query = <(&Item, &Name, &Carried)>::query();// (2)
    let mut y = 3;// (3)
    item_query
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player)// (4)
        .for_each(|(_, name, _)| {
            draw_batch.print(// (5)
                Point::new(3, y), 
                format!("{} : {}", y-2, &name.0)
            );
            y += 1;
        }
    );
    if y > 3 {// (6)
        draw_batch.print_color(Point::new(3, 2), "Items carried", 
            ColorPair::new(YELLOW, BLACK)
        );
    }

    draw_batch.submit(10000).expect("Batch error");
}
