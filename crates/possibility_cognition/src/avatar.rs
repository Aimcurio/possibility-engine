use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AvatarAttributes {
    pub name: String,
    pub curiosity: f32,        // [0.0, 1.0]
    pub dexterity: f32,        // [0.0, 1.0]
    pub efficiency: f32,       // [0.0, 1.0]
    pub patience: f32,         // [0.0, 1.0]
    pub speed_multiplier: f32, // [0.5, 2.5]
}

impl Default for AvatarAttributes {
    fn default() -> Self {
        Self {
            name: "Avatar".to_string(),
            curiosity: 0.7,
            dexterity: 0.8,
            efficiency: 0.85,
            patience: 0.9,
            speed_multiplier: 1.0,
        }
    }
}

impl AvatarAttributes {
    pub fn scientist(name: &str) -> Self {
        Self {
            name: name.to_string(),
            curiosity: 0.95,
            dexterity: 0.85,
            efficiency: 0.90,
            patience: 0.95,
            speed_multiplier: 1.1,
        }
    }

    pub fn engineer(name: &str) -> Self {
        Self {
            name: name.to_string(),
            curiosity: 0.75,
            dexterity: 0.95,
            efficiency: 0.95,
            patience: 0.85,
            speed_multiplier: 1.0,
        }
    }

    pub fn explorer(name: &str) -> Self {
        Self {
            name: name.to_string(),
            curiosity: 0.90,
            dexterity: 0.70,
            efficiency: 0.80,
            patience: 0.60,
            speed_multiplier: 1.3,
        }
    }

    pub fn apply_nlp_modifications(&mut self, text: &str) {
        let lower = text.to_lowercase();
        if lower.contains("fast") || lower.contains("speedy") || lower.contains("swift") {
            self.speed_multiplier = 1.8;
        } else if lower.contains("slow") || lower.contains("careful") {
            self.speed_multiplier = 0.7;
        }

        if lower.contains("curious") {
            self.curiosity = 0.98;
        }
        if lower.contains("dexterous") || lower.contains("nimble") {
            self.dexterity = 0.98;
        }
        if lower.contains("efficient") || lower.contains("optimal") {
            self.efficiency = 0.99;
        }
        if lower.contains("patient") {
            self.patience = 0.99;
        }
    }
}
