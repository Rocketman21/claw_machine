use amethyst::prelude::*;
use amethyst::ecs::{World, Entity};
use amethyst::renderer::Camera;
use amethyst::core::{
	Transform,
	math::Vector3,
};
use amethyst::window::ScreenDimensions;
use crate::systems::WASDMovement;

pub fn add_camera(world: &mut World) -> Entity {
	let mut transform = Transform::default();
	transform.set_translation_xyz(0., 4., 5.);
	transform.face_towards(Vector3::new(0.0, 2.0, 0.0), Vector3::new(0.0, 1.0, 0.0));

	let (width, height) = {
		let dimension = world.read_resource::<ScreenDimensions>();
		(dimension.width(), dimension.height())
	};

	world
		.create_entity()
		.with(Camera::standard_3d(width, height))
		.with(transform)
		.with(WASDMovement::default())
		.build()
}