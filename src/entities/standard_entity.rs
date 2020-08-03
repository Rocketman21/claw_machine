use amethyst::{
  ecs::{World, WorldExt, Builder, EntityBuilder},
  core::{
    Transform,
    math::{Vector3},
  },
	renderer::{
		shape::Shape,
		rendy::{
			mesh::{Normal, Position, Tangent, TexCoord},
		},
		palette::{LinSrgba},
	},
};
use amethyst_physics::{
  servers::{RigidBodyDesc, PhysicsWorld, ShapeDesc},
  PtReal,
};
use crate::component_creators::{
  create_mesh,
  create_material
};

pub struct PhysicalEntityBuilder<N: PtReal> {
  pub position: (f32, f32, f32),
  pub scale: (f32, f32, f32),
  pub shape: Shape,
  pub physical_shape: ShapeDesc<N>,
  pub rigid_body: RigidBodyDesc<N>,
  pub color: LinSrgba,
}

impl Default for PhysicalEntityBuilder<f32> {
  fn default() -> Self {
    PhysicalEntityBuilder {
      position: (0., 0., 0.),
      shape: Shape::Cube,
      scale: (1., 1., 1.),
      physical_shape: ShapeDesc::Cube {
        half_extents: Vector3::new(1., 1., 1.),
      },
      rigid_body: RigidBodyDesc::default(),
      color: LinSrgba::default(),
    }
  }
}

impl<'a> PhysicalEntityBuilder<f32> {
  pub fn new(self, world: &'a mut World) -> EntityBuilder {
    let mut transform = Transform::default();
    transform.set_translation_xyz(self.position.0, self.position.1, self.position.2);
  
    let mesh = {
      let mesh_data = self.shape
        .generate::<(
          Vec<Position>,
          Vec<Normal>,
          Vec<Tangent>,
          Vec<TexCoord>
        )>(Some(self.scale))
        .into();
  
      create_mesh(world, mesh_data)
    };
  
    let material = create_material(world, self.color, 0., 1.);
  
    let shape = {
      let physics_world = world.fetch::<PhysicsWorld<f32>>();
  
      physics_world.shape_server().create(&self.physical_shape)
    };
  
    let rigid_body = {
      let physics_world = world.fetch::<PhysicsWorld<f32>>();
  
      physics_world.rigid_body_server().create(&self.rigid_body)
    };
    
    world
      .create_entity()
      .with(transform)
      .with(mesh)
      .with(material)
      .with(shape)
      .with(rigid_body)
  }
}
