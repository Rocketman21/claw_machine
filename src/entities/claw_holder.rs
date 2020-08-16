use amethyst::{
	prelude::*,
	ecs::{
			World,
			Entity,
	},
	core::{Transform, Parent},
	renderer::{
		shape::Shape,
		rendy::mesh::{Normal, Position, Tangent, TexCoord},
		palette::LinSrgba,
	},
};
use crate::{
	systems::WASDMovement,
	component_creators::{
		create_mesh,
		create_material
	}
};

pub fn add_claw_holder(world: &mut World, parent: Entity) -> Entity {
	let mut transform = Transform::default();
	transform.set_translation_xyz(0.5, 3.6, 0.5);

	let scale = (0.1, 0.1, 0.2);

	let mesh = {
		let mesh_data = Shape::Cube
			.generate::<(
				Vec<Position>,
				Vec<Normal>,
				Vec<Tangent>,
				Vec<TexCoord>
			)>(Some(scale))
			.into();

		create_mesh(world, mesh_data)
	};

	let material = create_material(world, LinSrgba::new(255., 255., 255., 1.), 1., 1.);

	world
		.create_entity()
		.with(transform)
		.with(mesh)
		.with(material)
		.with(Parent { entity: parent })
		.with(WASDMovement {
			speed: 0.03,
			..WASDMovement::new_with_boundaries(-0.5, 3.6, -0.5, 1.)
		})
		.build()
}