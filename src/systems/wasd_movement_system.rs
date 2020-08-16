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
use amethyst::core::{math::Vector3, Transform};
use amethyst::input::{InputHandler, StringBindings};

#[derive(Default)]
pub struct ControlledEntity {
	index: usize,
}

pub struct CubicBoundaries {
	position: Vector3<f32>,
	size: f32,
}

impl CubicBoundaries {
	fn get_end(&self) -> Vector3<f32> {
		self.position.map(|boundary| boundary + self.size)
	}
}

pub struct WASDMovement {
	pub speed: f32,
	pub boundaries: Option<CubicBoundaries>
}

impl Default for WASDMovement {
	fn default() -> Self {
		Self {
			speed: 0.1,
			boundaries: None
		}
	}
}

impl WASDMovement {
	pub fn new_with_boundaries(x: f32, y: f32, z: f32, size: f32) -> Self {
		Self {
			speed: 0.1,
			boundaries: Some(CubicBoundaries {
				position: Vector3::new(x, y, z),
				size
			})
		}
	}
}

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
			.for_each(|(index, (transform, movement))| {
				if index == controlled_entity.index {
					if let (Some(x), Some(y), Some(z)) = (
						input.axis_value("x"),
						input.axis_value("y"),
						input.axis_value("z"),
					) {
						let next_position = transform.translation()
							+ Vector3::new(x, y, z).map(|amount| amount * movement.speed);

						let should_move = match &movement.boundaries {
							None => true,
							Some(boundary) => {
								next_position >= boundary.position
								&& next_position <= boundary.get_end()
							}
						};

						if should_move {
							transform.set_translation(next_position);
						}
					};
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
