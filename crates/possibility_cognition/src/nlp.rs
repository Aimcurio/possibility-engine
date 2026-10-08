use crate::avatar::AvatarAttributes;
use crate::goals::AvatarGoal;

pub struct NaturalLanguageInterpreter;

impl NaturalLanguageInterpreter {
    pub fn interpret_instruction(input: &str) -> Result<AvatarGoal, String> {
        let text = input.trim().to_lowercase();
        if text.is_empty() {
            return Err("Empty instruction".to_string());
        }

        // Scenario A intents
        if text.contains("cereal")
            || text.contains("breakfast")
            || (text.contains("bowl") && text.contains("make"))
            || (text.contains("pour") && text.contains("milk"))
        {
            return Ok(AvatarGoal::PrepareBowlOfCereal);
        }

        // Scenario B intents: Start gravity
        if (text.contains("gravity") || text.contains("graviton"))
            && (text.contains("start")
                || text.contains("activate")
                || text.contains("turn on")
                || text.contains("enable")
                || text.contains("run"))
        {
            return Ok(AvatarGoal::ActivateGravityExperiment);
        }

        // Scenario B intents: Stop gravity
        if (text.contains("gravity") || text.contains("graviton"))
            && (text.contains("stop")
                || text.contains("deactivate")
                || text.contains("turn off")
                || text.contains("disable")
                || text.contains("shutdown"))
        {
            return Ok(AvatarGoal::DeactivateGravityExperiment);
        }

        // Reset intents
        if text.contains("reset") || text.contains("restart") {
            return Ok(AvatarGoal::ResetLab);
        }

        // Cancellation / Idle intents
        if text.contains("stop")
            || text.contains("halt")
            || text.contains("pause")
            || text.contains("idle")
            || text.contains("cancel")
            || text.contains("wait")
        {
            return Ok(AvatarGoal::Idle);
        }

        Err(format!("Unrecognized instruction: '{}'", input))
    }

    pub fn interpret_attributes(input: &str, attrs: &mut AvatarAttributes) -> bool {
        let text = input.trim().to_lowercase();
        let mut modified = false;

        if text.contains("fast") || text.contains("speedy") || text.contains("swift") {
            attrs.speed_multiplier = 1.8;
            modified = true;
        } else if text.contains("slow") || text.contains("careful") {
            attrs.speed_multiplier = 0.7;
            modified = true;
        }

        if text.contains("curious") {
            attrs.curiosity = 0.98;
            modified = true;
        }
        if text.contains("dexterous") || text.contains("nimble") {
            attrs.dexterity = 0.98;
            modified = true;
        }
        if text.contains("efficient") || text.contains("optimal") {
            attrs.efficiency = 0.99;
            modified = true;
        }
        if text.contains("patient") {
            attrs.patience = 0.99;
            modified = true;
        }

        modified
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nlp_interpretation() {
        assert_eq!(
            NaturalLanguageInterpreter::interpret_instruction("Please make a bowl of cereal")
                .unwrap(),
            AvatarGoal::PrepareBowlOfCereal
        );
        assert_eq!(
            NaturalLanguageInterpreter::interpret_instruction("prepare breakfast").unwrap(),
            AvatarGoal::PrepareBowlOfCereal
        );
        assert_eq!(
            NaturalLanguageInterpreter::interpret_instruction(
                "activate artificial gravity experiment"
            )
            .unwrap(),
            AvatarGoal::ActivateGravityExperiment
        );
        assert_eq!(
            NaturalLanguageInterpreter::interpret_instruction("shutdown gravity field").unwrap(),
            AvatarGoal::DeactivateGravityExperiment
        );
        assert_eq!(
            NaturalLanguageInterpreter::interpret_instruction("stop immediately").unwrap(),
            AvatarGoal::Idle
        );
    }

    #[test]
    fn test_nlp_attribute_tuning() {
        let mut attrs = AvatarAttributes::default();
        let changed = NaturalLanguageInterpreter::interpret_attributes(
            "Make this avatar swift and curious",
            &mut attrs,
        );
        assert!(changed);
        assert_eq!(attrs.speed_multiplier, 1.8);
        assert_eq!(attrs.curiosity, 0.98);
    }
}
