use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ControlMode {
    Human,
    AutonomousAI,
    DirectedAI,
    Copilot,
    Scripted,
}

impl ControlMode {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Human => "Human (Direct)",
            Self::AutonomousAI => "Autonomous AI",
            Self::DirectedAI => "Directed AI",
            Self::Copilot => "Copilot (Assisted)",
            Self::Scripted => "Scripted / Replay",
        }
    }

    pub fn is_player_driven(&self) -> bool {
        matches!(self, Self::Human | Self::Copilot)
    }

    pub fn is_ai_driven(&self) -> bool {
        matches!(self, Self::AutonomousAI | Self::DirectedAI | Self::Copilot)
    }
}
