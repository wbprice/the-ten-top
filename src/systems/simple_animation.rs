use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::components::SimpleAnimation;

pub struct SimpleAnimationSystem;

impl<'s> System<'s> for SimpleAnimationSystem {
    type SystemData = (
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, SimpleAnimation>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut sprite_renders, mut animations, time): Self::SystemData) {
        for (sprite_render, anim) in (&mut sprite_renders, &mut animations).join() {
            anim.elapsed_time += time.delta_seconds();
            let frame_count = (anim.elapsed_time / anim.time_per_frame) as usize % anim.frames;
            if frame_count != anim.current_frame {
                anim.current_frame = frame_count;
                sprite_render.sprite_number = frame_count;
            }
            println!("{}", frame_count);
        }
    }
}
