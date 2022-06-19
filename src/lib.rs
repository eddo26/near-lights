use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Light {
    color: i8,
}

// Default light color is Red
impl Default for Light {
    fn default() -> Self {
        Self { color: -1 } // -1 is OFF
    }
}

#[near_bindgen]
impl Light {
    // Returns the current color of the light
    pub fn get_color(&self) -> String {
        match self.color {
            0 => "Red".to_string(),
            1 => "Yellow".to_string(),
            2 => "Green".to_string(),
            _ => "OFF".to_string(),
        }
    }

    // Sets the light to the color, determined by color_code
    pub fn set_color(&mut self, color_code: i8) {
        match color_code {
            0 => self.color = 0, // Red
            1 => self.color = 1, // Yellow
            2 => self.color = 2, // Green
            _ => {
                log!("received invalid color code! turning light OFF");
                self.color = -1; // OFF
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_default_color() {
        let light = Light::default();
        assert_eq!(light.get_color(), "OFF".to_string());
    }

    #[test]
    fn get_color_red() {
        let light = Light { color: 0 };
        assert_eq!(light.get_color(), "Red".to_string());
    }

    #[test]
    fn get_color_yellow() {
        let light = Light { color: 1 };
        assert_eq!(light.get_color(), "Yellow".to_string());
    }

    #[test]
    fn get_color_green() {
        let light = Light { color: 2 };
        assert_eq!(light.get_color(), "Green".to_string());
    }

    #[test]
    fn set_color_red() {
        let mut light = Light::default();
        light.set_color(0);
        assert_eq!(light.get_color(), "Red".to_string());
    }

    #[test]
    fn set_color_yellow() {
        let mut light = Light::default();
        light.set_color(1);
        assert_eq!(light.get_color(), "Yellow".to_string());
    }

    #[test]
    fn set_color_green() {
        let mut light = Light::default();
        light.set_color(2);
        assert_eq!(light.get_color(), "Green".to_string());
    }

    #[test]
    fn set_color_off() {
        let mut light = Light { color: 0 };
        light.set_color(-1);
        assert_eq!(light.get_color(), "OFF".to_string());
    }
}
