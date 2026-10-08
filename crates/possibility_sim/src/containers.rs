use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Container {
    pub capacity_ml: f32,
    pub cereal_grams: f32,
    pub milk_ml: f32,
    pub is_open: bool,
}

impl Container {
    pub fn new(capacity_ml: f32, cereal_grams: f32, milk_ml: f32, is_open: bool) -> Self {
        Self {
            capacity_ml,
            cereal_grams,
            milk_ml,
            is_open,
        }
    }

    pub fn new_bowl() -> Self {
        Self::new(750.0, 0.0, 0.0, true)
    }

    pub fn new_cereal_box() -> Self {
        Self::new(1000.0, 500.0, 0.0, true)
    }

    pub fn new_milk_carton() -> Self {
        Self::new(1000.0, 0.0, 1000.0, true)
    }

    pub fn current_liquid_and_cereal_volume(&self) -> f32 {
        // Approximate density of cereal grains: 0.5 g/ml -> 1g ~ 2ml volume
        self.milk_ml + (self.cereal_grams * 1.5)
    }

    pub fn dispense_cereal(
        &mut self,
        target: &mut Container,
        amount_g: f32,
    ) -> Result<f32, String> {
        if !self.is_open {
            return Err("Source container lid is closed".to_string());
        }
        if !target.is_open {
            return Err("Target container lid is closed".to_string());
        }
        if amount_g <= 0.0 {
            return Ok(0.0);
        }

        let actual_transfer = amount_g.min(self.cereal_grams);
        if actual_transfer <= 0.0 {
            return Err("No cereal remaining in source container".to_string());
        }

        let target_available =
            (target.capacity_ml - target.current_liquid_and_cereal_volume()).max(0.0);
        let target_max_cereal = target_available / 1.5;
        let final_transfer = actual_transfer.min(target_max_cereal);

        if final_transfer <= 0.0 {
            return Err("Target container is full".to_string());
        }

        self.cereal_grams -= final_transfer;
        target.cereal_grams += final_transfer;
        Ok(final_transfer)
    }

    pub fn pour_milk(&mut self, target: &mut Container, amount_ml: f32) -> Result<f32, String> {
        if !self.is_open {
            return Err("Source container is closed".to_string());
        }
        if !target.is_open {
            return Err("Target container is closed".to_string());
        }
        if amount_ml <= 0.0 {
            return Ok(0.0);
        }

        let actual_transfer = amount_ml.min(self.milk_ml);
        if actual_transfer <= 0.0 {
            return Err("No milk remaining in source container".to_string());
        }

        let target_available =
            (target.capacity_ml - target.current_liquid_and_cereal_volume()).max(0.0);
        let final_transfer = actual_transfer.min(target_available);

        if final_transfer <= 0.0 {
            return Err("Target container is full".to_string());
        }

        self.milk_ml -= final_transfer;
        target.milk_ml += final_transfer;
        Ok(final_transfer)
    }

    pub fn is_prepared_cereal(&self) -> bool {
        self.cereal_grams >= 25.0 && self.milk_ml >= 50.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cereal_preparation_flow() {
        let mut bowl = Container::new_bowl();
        let mut cereal_box = Container::new_cereal_box();
        let mut milk_carton = Container::new_milk_carton();

        assert!(!bowl.is_prepared_cereal());

        // Dispense cereal
        let dispensed = cereal_box
            .dispense_cereal(&mut bowl, 40.0)
            .expect("cereal transfer");
        assert_eq!(dispensed, 40.0);
        assert_eq!(bowl.cereal_grams, 40.0);
        assert!(!bowl.is_prepared_cereal());

        // Pour milk
        let poured = milk_carton
            .pour_milk(&mut bowl, 100.0)
            .expect("milk transfer");
        assert_eq!(poured, 100.0);
        assert_eq!(bowl.milk_ml, 100.0);

        // Verification of finished state
        assert!(bowl.is_prepared_cereal());
    }

    #[test]
    fn test_capacity_overflow_prevention() {
        let mut small_cup = Container::new(100.0, 0.0, 0.0, true);
        let mut big_milk = Container::new_milk_carton();

        let poured = big_milk
            .pour_milk(&mut small_cup, 500.0)
            .expect("capped transfer");
        assert_eq!(poured, 100.0);
        assert_eq!(small_cup.milk_ml, 100.0);
        assert_eq!(big_milk.milk_ml, 900.0);
    }
}
