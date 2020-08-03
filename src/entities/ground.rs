use amethyst::{
  ecs::{World, Builder},
	renderer::{
		palette::{LinSrgba},
	},
};
use amethyst_physics::{
	servers::{RigidBodyDesc, BodyMode},
};
use crate::{
  entities::standard_entity::PhysicalEntityBuilder,
};

pub fn add_ground(world: &mut World) {
  let ground_builder = PhysicalEntityBuilder {
    scale: (20., 0.2, 20.),
    rigid_body: RigidBodyDesc {
      mode: BodyMode::Static,
      ..RigidBodyDesc::default()
    },
    color: LinSrgba::new(220. / 255., 20. / 255., 60. / 255., 1.),
    ..PhysicalEntityBuilder::default()
  };

  ground_builder.new(world).build();
}