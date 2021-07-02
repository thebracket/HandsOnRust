use crate::prelude::*;

#[system]
#[read_component(Point)]
#[write_component(FieldOfView)]
pub fn fov(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
) {
    let mut views = <(&Point, &mut FieldOfView)>::query();// (1)
    views
        .iter_mut(ecs)// (2)
        .filter(|(_, fov)| fov.is_dirty)// (3)
        .for_each(|(pos, mut fov)| {
            fov.visible_tiles = field_of_view_set(*pos, fov.radius, map);// (4)
            fov.is_dirty = false;// (5)
        }
    );
}
