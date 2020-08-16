use amethyst::{
	prelude::*,
	ecs::{Entity, World},
	core::Transform,
};

use crate::component_creators::create_gltf;

pub fn add_claw_machine(world: &mut World) -> Entity {
	let gltf_transform = Transform::default();
	let gltf = create_gltf(world, "mesh/claw_machine.glb");

	world
		.create_entity()
		.with(gltf_transform)
		.with(gltf)
		.build()
}