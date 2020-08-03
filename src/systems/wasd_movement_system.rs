use amethyst::ecs::{
    System,
    ReadStorage,
    WriteStorage,
    Read,
    Component,
    DenseVecStorage,
    Write,
    Join,
};
use amethyst::core::Transform;
use amethyst::input::{InputHandler, StringBindings};

#[derive(Default)]
pub struct ControlledEntity {
    index: usize,
}

pub struct WASDMovement;

impl Component for WASDMovement {
    type Storage = DenseVecStorage<Self>;
}

pub struct WASDMovementSystem;

impl<'s> System<'s> for WASDMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, WASDMovement>,
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, ControlledEntity>,
    );

    fn run(
        &mut self,
        (mut transforms, movements, input, mut controlled_entity): Self::SystemData
    ) {
        let entities_count = (&mut transforms, &movements).join().count();

        (&mut transforms, &movements)
            .join()
            .enumerate()
            .for_each(|(index, (transform, _movement))| {
                if index == controlled_entity.index {
                    if let Some(mv_amount) = input.axis_value("x") {
                        transform.prepend_translation_x(mv_amount * 0.5);
                    }
                    if let Some(mv_amount) = input.axis_value("y") {
                        transform.prepend_translation_y(mv_amount * 0.5);
                    }
                    if let Some(mv_amount) = input.axis_value("z") {
                        transform.prepend_translation_z(mv_amount * 0.5);
                    }
                }
            });

        if input.action_is_down("toggle_control").unwrap_or(false) {
            let mut next_index = controlled_entity.index + 1;

            if next_index == entities_count {
                next_index = 0;
            }

            controlled_entity.index = next_index;

            println!("Toggling movement control. Index changed to {}", next_index);
        }
    }
}
