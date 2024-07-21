use crate::types::{
	enums::*,
	structs::CraftingLevel,
	traits::{BuffAction, CraftingAction},
	Simulation,
};

#[derive(Clone)]
pub struct HeartAndSoul;

impl BuffAction for HeartAndSoul {
	fn get_duration(&self, _simulation_state: &Simulation) -> i32 {
		// basically infinity
		// improvement: fix for crafting rotations over 2,147,483,647 steps long
		i32::MAX
	}

	fn can_be_clipped(&self) -> bool {
		true
	}

	fn get_buff(&self) -> Buff {
		Buff::HeartAndSoul
	}

	fn get_initial_stacks(&self) -> u32 {
		0
	}

	fn get_tick(&self) -> Option<fn(&mut Simulation, &dyn CraftingAction) -> ()> {
		Some(|simulation_state, action| {
			let used_on_non_good_or_excellent = simulation_state.state() != StepState::Good
				&& simulation_state.state() != StepState::Excellent;
			use CraftingActionEnum as CA;
			if used_on_non_good_or_excellent
				&& [
					CA::PreciseTouch,
					CA::IntensiveSynthesis,
					CA::TricksOfTheTrade,
				]
				.contains(&action.get_enum())
			{
				simulation_state.remove_buff(Buff::HeartAndSoul);
			}
		})
	}
}

impl CraftingAction for HeartAndSoul {
	fn skip_on_fail(&self) -> bool {
		true
	}

	fn get_level_requirement(&self) -> (CraftingJob, CraftingLevel) {
		(CraftingJob::Any, CraftingLevel::new(86).unwrap())
	}

	fn get_type(&self) -> ActionType {
		ActionType::Other
	}

	fn _get_success_rate(&self, _simulation_state: &Simulation) -> u32 {
		100
	}

	fn _can_be_used(&self, simulation_state: &Simulation, _linear: Option<bool>) -> bool {
		simulation_state.crafter_stats.specialist
			&& !simulation_state
				.steps
				.iter()
				.any(|s| s.action.get_enum() == CraftingActionEnum::HeartAndSoul)
	}

	fn get_base_cp_cost(&self, _simulation_state: &Simulation) -> u32 {
		0
	}

	fn get_durability_cost(&self, _simulation_state: &Simulation) -> u32 {
		0
	}

	fn execute_with_flags(
		&self,
		simulation_state: &mut Simulation,
		_safe: bool,
		_skip_stack_addition: bool,
	) {
		self.get_overrides()
			.into_iter()
			.for_each(|b| simulation_state.remove_buff(b));
		simulation_state.add_buff(self.get_applied_buff(simulation_state));
	}

	fn skips_buff_ticks(&self) -> bool {
		true
	}

	fn get_enum(&self) -> CraftingActionEnum {
		CraftingActionEnum::HeartAndSoul
	}
}
