use crate::handoff::{HandoffRecord, InterruptionPolicy};
use crate::modes::ControlMode;
use possibility_cognition::{
    AvatarAttributes, AvatarGoal, DeterministicPlanner, PerceptionSnapshot, TaskStep,
};
use possibility_core::{AvatarId, SimCommand};
use possibility_sim::SimWorld;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AvatarControlState {
    pub avatar_id: AvatarId,
    pub mode: ControlMode,
    pub goal: AvatarGoal,
    pub suspended_goal: Option<AvatarGoal>,
    pub attributes: AvatarAttributes,
    pub handoff_history: Vec<HandoffRecord>,
}

pub struct ControlArbitrator {
    pub states: HashMap<AvatarId, AvatarControlState>,
    pub planner: DeterministicPlanner,
}

impl Default for ControlArbitrator {
    fn default() -> Self {
        Self::new()
    }
}

impl ControlArbitrator {
    pub fn new() -> Self {
        let mut states = HashMap::new();

        // Avatar 0: Alpha (Directed AI)
        states.insert(
            AvatarId(0),
            AvatarControlState {
                avatar_id: AvatarId(0),
                mode: ControlMode::DirectedAI,
                goal: AvatarGoal::PrepareBowlOfCereal,
                suspended_goal: None,
                attributes: AvatarAttributes::scientist("Alpha"),
                handoff_history: Vec::new(),
            },
        );

        // Avatar 1: Beta (Autonomous AI)
        states.insert(
            AvatarId(1),
            AvatarControlState {
                avatar_id: AvatarId(1),
                mode: ControlMode::AutonomousAI,
                goal: AvatarGoal::ActivateGravityExperiment,
                suspended_goal: None,
                attributes: AvatarAttributes::engineer("Beta"),
                handoff_history: Vec::new(),
            },
        );

        // Avatar 2: Gamma (Human direct control)
        states.insert(
            AvatarId(2),
            AvatarControlState {
                avatar_id: AvatarId(2),
                mode: ControlMode::Human,
                goal: AvatarGoal::Idle,
                suspended_goal: None,
                attributes: AvatarAttributes::explorer("Gamma"),
                handoff_history: Vec::new(),
            },
        );

        Self {
            states,
            planner: DeterministicPlanner::new(),
        }
    }

    pub fn get_mode(&self, avatar_id: AvatarId) -> Option<ControlMode> {
        self.states.get(&avatar_id).map(|s| s.mode)
    }

    pub fn get_goal(&self, avatar_id: AvatarId) -> Option<&AvatarGoal> {
        self.states.get(&avatar_id).map(|s| &s.goal)
    }

    pub fn assign_goal(&mut self, avatar_id: AvatarId, goal: AvatarGoal) -> Result<(), String> {
        let state = self
            .states
            .get_mut(&avatar_id)
            .ok_or_else(|| format!("Avatar {:?} not found", avatar_id))?;
        state.goal = goal;
        Ok(())
    }

    pub fn switch_mode(
        &mut self,
        avatar_id: AvatarId,
        new_mode: ControlMode,
        world: &SimWorld,
        policy: InterruptionPolicy,
    ) -> Result<HandoffRecord, String> {
        let avatar_rec = world
            .get_avatar(avatar_id)
            .ok_or_else(|| format!("Avatar {:?} not in world", avatar_id))?;
        let avatar_pos = avatar_rec.position;
        let held = avatar_rec.grasp.held_object;

        let state = self
            .states
            .get_mut(&avatar_id)
            .ok_or_else(|| format!("Avatar {:?} control state not found", avatar_id))?;

        let old_mode = state.mode;
        if old_mode == new_mode {
            return Ok(HandoffRecord {
                avatar_id,
                previous_mode: old_mode,
                new_mode,
                avatar_position: avatar_pos,
                held_object: held,
                preserved_goal: Some(state.goal.clone()),
                timestamp: world.time_elapsed,
                state_preserved: true,
            });
        }

        // Interruption handling
        let preserved_goal = if old_mode.is_ai_driven() && !new_mode.is_ai_driven() {
            match policy {
                InterruptionPolicy::SuspendAndPreserve => {
                    state.suspended_goal = Some(state.goal.clone());
                    Some(state.goal.clone())
                }
                InterruptionPolicy::CancelGoal => {
                    state.suspended_goal = None;
                    state.goal = AvatarGoal::Idle;
                    None
                }
            }
        } else if !old_mode.is_ai_driven() && new_mode.is_ai_driven() {
            // Resuming AI control: check if there is a suspended goal to resume
            if let Some(resumed) = state.suspended_goal.take() {
                state.goal = resumed.clone();
                Some(resumed)
            } else {
                Some(state.goal.clone())
            }
        } else {
            Some(state.goal.clone())
        };

        state.mode = new_mode;

        let record = HandoffRecord {
            avatar_id,
            previous_mode: old_mode,
            new_mode,
            avatar_position: avatar_pos,
            held_object: held,
            preserved_goal,
            timestamp: world.time_elapsed,
            state_preserved: true,
        };

        state.handoff_history.push(record.clone());
        Ok(record)
    }

    pub fn tick_avatar(
        &mut self,
        avatar_id: AvatarId,
        world: &SimWorld,
        player_cmd: Option<SimCommand>,
    ) -> Option<SimCommand> {
        let state = self.states.get_mut(&avatar_id)?;

        match state.mode {
            ControlMode::Human => player_cmd,

            ControlMode::AutonomousAI | ControlMode::DirectedAI => {
                let snapshot = PerceptionSnapshot::capture(world, avatar_id)?;
                let speed = state.attributes.speed_multiplier * 3.0;

                let next_step = self.planner.plan_next_step(&state.goal, &snapshot);

                if matches!(next_step, TaskStep::Complete) {
                    if state.mode == ControlMode::AutonomousAI {
                        // Autonomous AI chooses next goal if idle or complete
                        if state.goal == AvatarGoal::PrepareBowlOfCereal {
                            state.goal = AvatarGoal::ActivateGravityExperiment;
                        } else if state.goal == AvatarGoal::ActivateGravityExperiment {
                            state.goal = AvatarGoal::Idle;
                        }
                    }
                    return None;
                }

                self.planner.step_to_command(avatar_id, &next_step, speed)
            }

            ControlMode::Copilot => {
                // If player provides an explicit command, follow player
                if let Some(cmd) = player_cmd {
                    Some(cmd)
                } else {
                    // Copilot AI assists with active goal when player is not actively pressing input
                    let snapshot = PerceptionSnapshot::capture(world, avatar_id)?;
                    let speed = state.attributes.speed_multiplier * 2.5;
                    let step = self.planner.plan_next_step(&state.goal, &snapshot);
                    self.planner.step_to_command(avatar_id, &step, speed)
                }
            }

            ControlMode::Scripted => player_cmd,
        }
    }
}
