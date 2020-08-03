use amethyst::prelude::*;
use amethyst::ecs::World;
use amethyst::renderer::{
	light::{DirectionalLight, Light},
	palette::Srgb,
};
use amethyst::core::{
	Transform,
	math::{Vector3},
};

pub fn add_light(world: &mut World, direction: Vector3<f32>) {
	let mut transform = Transform::default();
	transform.set_translation_xyz(6.0, 6.0, 6.0);

	let light: Light = DirectionalLight {
		color: Srgb::new(1.0, 1.0, 1.0),
		direction: direction.normalize(),
		intensity: 5.0,
	}.into();

	world
		.create_entity()
		.with(light)
		.with(transform)
		.build();
}