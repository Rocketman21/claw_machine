use amethyst::prelude::*;
use amethyst::input::{VirtualKeyCode, is_key_down};
use amethyst::core::math::Vector3;
use amethyst::winit;

use crate::entities::{
	add_light,
	add_camera,
	Player,
	add_player,
	add_claw_machine,
	add_claw_holder,
	add_claw,
	add_ground,
};

fn handle_esc(event: &winit::Event, is_paused: bool) -> SimpleTrans {
	if is_key_down(&event, VirtualKeyCode::Escape) {
		if is_paused {
			return Trans::Pop;
		} else {
			return Trans::Push(Box::new(PauseState));
		}
	}

	Trans::None
}

fn handle_cmd_q(event: &winit::Event) -> SimpleTrans {
	if is_key_down(&event, VirtualKeyCode::Q) {
		return Trans::Quit;
	}

	Trans::None
}

pub struct GameplayState;
pub struct PauseState;

impl SimpleState for GameplayState {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		add_camera(data.world);
		add_ground(data.world);
		data.world.register::<Player>();
		// add_player(data.world);
		add_light(data.world, Vector3::new(-1.0, -1.0, -1.0));
		let claw_machine = add_claw_machine(data.world);
		let claw_holder = add_claw_holder(data.world, claw_machine);
		add_claw(data.world, claw_holder);

		println!("Hello gameplay!");
	}

	fn on_resume(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
		println!("Back ogain!");
	}

	fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
		let mut trans = Trans::None;

		if let StateEvent::Window(event) = &event {
			trans = handle_esc(&event, false);

			if let Trans::None = trans {
				trans = handle_cmd_q(&event);
			}
		}

		trans
	}
}

impl SimpleState for PauseState {
	fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
		println!("Pause");
	}

	fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
		let mut trans = Trans::None;
		
		if let StateEvent::Window(event) = &event {
			trans = handle_esc(&event, true);
		}

		trans
	}
}
