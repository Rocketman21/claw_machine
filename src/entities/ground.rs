use amethyst::{
	ecs::{World, Builder},
	renderer::palette::LinSrgba,
};
use amethyst_physics::servers::{RigidBodyDesc, BodyMode};
use crate::entities::standard_entity::PhysicalEntityBuilder;

pub fn add_ground(world: &mut World) {
	let ground_builder = PhysicalEntityBuilder {
		scale: (10., 0.2, 10.),
		rigid_body: RigidBodyDesc {
			mode: BodyMode::Static,
			..RigidBodyDesc::default()
		},
		color: LinSrgba::new(200. / 255., 200. / 255., 20. / 255., 1.),
		..PhysicalEntityBuilder::default()
	};

	ground_builder.new(world).build();
}