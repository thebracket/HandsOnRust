use crate::prelude::*;

pub fn spawn_player(ecs : &mut World, pos : Point) { // (1)
    ecs.push(// (2)
        (
            Player,// (3)
            pos, // (4)
            Render{// (5)
                color: ColorPair::new(WHITE, BLACK),
                glyph : to_cp437('@')
            }
        )
    );
}
