use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();// (1)
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);

    <(&Point, &Render)>::query()// (2)
        .iter(ecs)// (3)
        .for_each(|(pos, render)| {// (4)
            draw_batch.set(// (5)
                *pos - offset,
                render.color,
                render.glyph
            );
        }
    );
    draw_batch.submit(5000).expect("Batch error");// (6)
}
