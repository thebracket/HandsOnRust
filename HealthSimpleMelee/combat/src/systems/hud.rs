use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn hud(ecs: &SubWorld) {
    let mut health_query = <&Health>::query().filter(component::<Player>());// (1)
    let player_health = health_query
        .iter(ecs)
        .nth(0) // (2)
        .unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);// (3)
    draw_batch.print_centered(1, 
        "Explore the Dungeon. Cursor keys to move.");// (4)
    draw_batch.bar_horizontal(// (5)
        Point::zero(),// (6)
        SCREEN_WIDTH*2,// (7)
        player_health.current,// (8)
        player_health.max,// (9)
        ColorPair::new(RED, BLACK)// (10)
    );
    draw_batch.print_color_centered(
        0,
        format!(" Health: {} / {} ", 
            player_health.current, 
            player_health.max
        ),
        ColorPair::new(WHITE, RED)
    );
    draw_batch.submit(10000).expect("Batch error");
}
