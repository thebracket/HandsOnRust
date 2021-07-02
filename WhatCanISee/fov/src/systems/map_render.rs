use crate::prelude::*;

#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn map_render(
    ecs: &SubWorld,
    #[resource] map: &Map,
    #[resource] camera:&Camera
) {
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());// (1)
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    let player_fov = fov.iter(ecs).nth(0).unwrap();

    for y in camera.top_y ..= camera.bottom_y {
        for x in camera.left_x .. camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            if map.in_bounds(pt) && player_fov.visible_tiles.contains(&pt) {
                let idx = map_idx(x, y);
                match map.tiles[idx] {
                    TileType::Floor => {
                        draw_batch.set(
                            pt - offset,
                            ColorPair::new(
                                WHITE,
                                BLACK
                            ),
                            to_cp437('.')
                        );
                    }
                    TileType::Wall => {
                        draw_batch.set(
                            pt - offset, 
                            ColorPair::new(
                                WHITE, 
                                BLACK
                            ),
                            to_cp437('#')
                        );
                    }
                }
            }
        }
    }
    draw_batch.submit(0).expect("Batch error");
}
