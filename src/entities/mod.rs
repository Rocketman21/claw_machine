pub use self::{
	light::add_light,
	camera::add_camera,
	player::Player,
	player::add_player,
	claw_machine::add_claw_machine,
	claw_holder::add_claw_holder,
	claw::add_claw,
	ground::add_ground,
	standard_entity::PhysicalEntityBuilder,
};

mod light;
mod camera;
mod player;
mod claw_machine;
mod claw_holder;
mod claw;
mod ground;
mod standard_entity;