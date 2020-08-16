use amethyst::{
	ecs::{Component, DenseVecStorage, World, Builder, Entity},
	renderer::{
		shape::Shape,
		palette::LinSrgba,
	},
	core::Transform,
};
use amethyst_physics::{
	servers::{RigidBodyDesc, ShapeDesc},
};
use crate::{
	entities::standard_entity::PhysicalEntityBuilder,
	systems::WASDMovement
};


pub struct Player {
	size: (f32, f32, f32),
}

impl Player {
	fn new() -> Player {
		Player {
			size: (0.5, 0.5, 0.5),
		}
	}
}

impl Component for Player {
	type Storage = DenseVecStorage<Self>;
}

pub fn add_player(world: &mut World) -> Entity {
	let player = Player::new();
	let mut transform = Transform::default();
	transform.set_translation_y(0.20);

	let player_builder = PhysicalEntityBuilder {
		scale: player.size,
		transform,
		color: LinSrgba::new(1.0, 0.5, 0.0, 1.0),
		shape: Shape::Sphere(32, 32),
		physical_shape: ShapeDesc::Sphere {
			radius: player.size.0 / 2.
		},
		rigid_body: RigidBodyDesc {
			mass: 2.0,
			bounciness: 1.0,
			friction: 0.05,
			..RigidBodyDesc::default()
		},
		..PhysicalEntityBuilder::default()
	};
	
	player_builder
		.new(world)
		.with(player)
		.with(WASDMovement::default())
		.build()
}
