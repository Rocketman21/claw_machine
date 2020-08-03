use amethyst::{
  ecs::{World, WorldExt},
  renderer::{
		types::{Mesh, MeshData, Texture},
		rendy::{
			texture::palette::load_from_linear_rgba,
		},
		mtl::{Material, MaterialDefaults},
		palette::{LinSrgba},
	},
	assets::{Loader, AssetStorage, Handle},
};

pub fn create_mesh(world: &mut World, mesh_data: MeshData) -> Handle<Mesh> {
	let loader = world.read_resource::<Loader>();
	let asset_storage = world.read_resource::<AssetStorage<Mesh>>();
	let mesh = loader.load_from_data(mesh_data, (), &asset_storage);

	mesh
}

pub fn create_material(
	world: &World,
	color: LinSrgba,
	metallic: f32,
	roughness: f32,
) -> Handle<Material> {
	let loader = world.read_resource::<Loader>();
	let asset_storage = world.read_resource::<AssetStorage<Texture>>();

	let albedo = loader.load_from_data(
		load_from_linear_rgba(color).into(),
		(),
		&asset_storage
	);
	let metallic_roughness = loader.load_from_data(
		load_from_linear_rgba(LinSrgba::new(0.0, roughness, metallic, 0.0)).into(),
		(),
		&asset_storage
	);

	let asset_storage = world.read_resource::<AssetStorage<Material>>();

	let material_defaults = world.read_resource::<MaterialDefaults>().0.clone();
	let material = loader.load_from_data(
		Material {
			albedo,
			metallic_roughness,
			..material_defaults
		},
		(),
		&asset_storage
	);

	material
}