use amethyst::{
	prelude::*,
	ecs::{
			World, WorldExt, Entity,
	},
	core::{Transform, Parent},
};

use crate::component_creators::create_gltf;

pub fn add_claw(world: &mut World, parent: Entity) {
	let mut transform = Transform::default();
	transform.set_translation_y(-0.6);

	let gltf = create_gltf(world, "mesh/claw.glb");

	world
		.create_entity()
		.with(transform)
		.with(gltf)
		.with(Parent { entity: parent })
		.build();
}
