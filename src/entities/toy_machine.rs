use amethyst::{
	prelude::*,
	assets::{Loader, AssetStorage},
	ecs::{
			World,
			WorldExt,
	},
	core::{
		Transform, 
		math::{Vector3},
	},
	gltf::{GltfSceneAsset, GltfSceneFormat},
};

pub fn add_toy_machine(world: &mut World) {
	let mut transform = Transform::default();
	transform.set_translation_xyz(0.0, 0.0, 0.0);
	transform.set_scale(Vector3::new(10.0, 10.0, 10.0));
	transform.set_rotation_y_axis(90.0);

	let gltf = {
		let loader = world.read_resource::<Loader>();
		let gltf_storage = world.read_resource::<AssetStorage<GltfSceneAsset>>();
		let gltf = loader.load(
			"mesh/claw_machine.glb",
			GltfSceneFormat::default(),
			(),
			&gltf_storage
		);
	
		gltf
	};

	world
		.create_entity()
		.with(transform)
		.with(gltf)
		.build();
}