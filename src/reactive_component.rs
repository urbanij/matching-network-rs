use quantiphy_rs::Quantity;
#[derive(Debug)]
enum ComponentType {
    L,
    C,
    Wire,
}

#[derive(Debug)]
pub struct ReactiveComponent<T> {
    reactance: T,
    component_type: ComponentType, 
    frequency: Option<T>,
}

impl ReactiveComponent<f32> {
    pub fn new(reactance: f32, frequency: Option<f32>) -> Self {
        let component_type = match reactance {
            r if r > 0.0 => ComponentType::L,
            r if r < 0.0 => ComponentType::C,
            _ => ComponentType::Wire,
        };
        ReactiveComponent {
            reactance,
            component_type,
            frequency,
        }
    }

    pub fn get_susceptance(&self) -> f32 {
        // Return equivalent susceptance value

        const USE_MILLISIEMENS: bool = false;

        match self.component_type {
            ComponentType::L | ComponentType::C => {
                if USE_MILLISIEMENS {
                    (-1.0 / self.reactance) * 1e3   // S -> mS
                } else {
                    -1.0 / self.reactance           // S
                }
            },
            ComponentType::Wire => std::f32::INFINITY,
        }
    }

    pub fn get_value(&self) -> Option<f32> {
		use std::f32::consts::PI;
    
        if let Some(f) = self.frequency {
            match self.component_type {
                ComponentType::L => {
                    Some(self.reactance / (2.0 * PI * f))
                },
                ComponentType::C => {
                    Some(-1.0 / (2.0 * PI * f * self.reactance))
                },
                ComponentType::Wire => Some(0.0),
            }
        } else {
            None
        }
    }

    pub fn get_freq(&self) -> Option<f32> {
        self.frequency
    }

    fn get_component_type_str(&self) -> &str {
        return match self.component_type {
            ComponentType::L => "Inductor",
            ComponentType::C => "Capacitor",
            ComponentType::Wire => "Wire", 
        };
    }

}


impl std::fmt::Display for ReactiveComponent<f32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        const USE_QUANTIPHY: bool = true;

        let mut ans;
        
        if USE_QUANTIPHY {
            ans = format!(
                "{}:\n    X = {}Ω ⇔ B = {}mS", 
                self.get_component_type_str(), 
                Quantity::from(self.reactance as f64),
                Quantity::from(self.get_susceptance() as f64), // TODO: FIX this
            );

            if let Some(f) = self.frequency {
                ans = format!(
                    "{}\n    {:?} = {}H (@ {}Hz)", 
                    ans, 
                    self.component_type, 
                    Quantity::from(self.get_value().unwrap() as f64), 
                    Quantity::from(f as f64),
                );
            }
        }
        else {
            ans = format!(
                "{}:\n    X = {:.5} Ω ⇔ B = {:.5} mS",  
                self.get_component_type_str(), 
                self.reactance,
                self.get_susceptance(),
            );

            if let Some(f) = self.frequency {
                ans = format!(
                    "{}\n    {:?} = {:.5} H (@ {:.5} Hz)", 
                    ans, 
                    self.component_type, 
                    self.get_value().unwrap(),
                    f,
                );
            }
        }
        write!(f, "{}", ans)
    }
}
