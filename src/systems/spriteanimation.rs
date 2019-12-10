use amethyst::{
    core::{Time, SystemDesc},
    derive::SystemDesc,
    ecs::{join::Join, Read, ReadStorage, WriteStorage, System, SystemData, World},
    renderer::SpriteRender,
};
use crate::d2::{SpriteAnimationComponent, SpriteCountComponent};

#[derive(Default,SystemDesc)]
pub struct SpriteAnimationSystem;

impl<'a> System<'a> for SpriteAnimationSystem {
    type SystemData = (
        Read<'a, Time>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, SpriteAnimationComponent>,
        ReadStorage<'a, SpriteCountComponent>,
    );

    fn run(&mut self, (time, mut write_sprites, mut write_spriteanimations, read_spritecounts): Self::SystemData) {
        for (mut sprite, sprite_animation, sprite_count) in (&mut write_sprites, &mut write_spriteanimations, &read_spritecounts).join() {
            if sprite_animation.update_rate == 0.0 {
                continue
            }
            if time.absolute_time_seconds() - sprite_animation.last_update >= sprite_animation.update_rate {
                sprite_animation.last_update = time.absolute_time_seconds();

                if sprite.sprite_number < sprite_count.count - 1 {
                    sprite.sprite_number += 1;
                } else {
                    sprite.sprite_number = 0;
                }
            }
        }
    }
}