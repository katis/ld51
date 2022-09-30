use bevy::prelude::*;

use std::time::Duration;

#[derive(Debug, Component)]
pub enum Animation {
    Start(Handle<AnimationTag>),
    Run {
        tag: Handle<AnimationTag>,
        frame: usize,
        timer: Timer,
    },
}

#[derive(Default, Debug, bevy::reflect::TypeUuid)]
#[uuid = "b20e47dd-8766-4e0d-9242-34d6e6e50f15"]
pub struct AnimationTag(pub Vec<Frame>);

#[derive(Debug, Clone)]
pub struct Frame {
    pub texture_index: usize,
    pub duration: Duration,
}

pub fn animation_system(
    time: Res<Time>,
    animation_tags: Res<Assets<AnimationTag>>,
    mut query: Query<(&mut Animation, &mut TextureAtlasSprite)>,
) {
    for (mut animation, mut sprite) in query.iter_mut() {
        match animation.as_mut() {
            Animation::Start(animation_handle) => {
                if let Some(tag) = animation_tags.get(animation_handle) {
                    *animation = Animation::Run {
                        tag: animation_handle.clone(),
                        frame: 0,
                        timer: Timer::new(tag.0[0].duration, false),
                    };
                }
            }
            Animation::Run { tag, frame, timer } => {
                timer.tick(time.delta());

                if timer.finished() {
                    if let Some(tag_def) = animation_tags.get(tag) {
                        *frame = (*frame + 1) % tag_def.0.len();
                        let frame_def = &tag_def.0[*frame];

                        sprite.index = frame_def.texture_index;
                        timer.set_duration(frame_def.duration);
                        timer.reset();
                    }
                }
            }
        }
    }
}
