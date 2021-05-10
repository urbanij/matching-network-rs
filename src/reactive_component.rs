#[derive(Debug)]
enum ComponentType {
    L,
    C,
    Wire,
}

#[derive(Debug)]
pub struct ReactiveComponent {
    reactance: f32,
    component_type: ComponentType, 
    frequency: Option<f32>,
}

impl ReactiveComponent {
    pub fn new(reactance: f32, frequency: Option<f32>) -> Self {
        let component_type = match reactance {
            r if r > 0.0 => ComponentType::L,
            r if r < 0.0 => ComponentType::C,
            _ => ComponentType::Wire,
        };
        ReactiveComponent {
            reactance: reactance,
            component_type: component_type,
            frequency: frequency,
        }
    }

    pub fn get_susceptance(&self) -> f32 {
        // Return equivalent susceptance value in milli Siemens.
        match self.component_type {
            ComponentType::L | ComponentType::C => (-1.0 / self.reactance) * 1e3,    // S -> mS
            ComponentType::Wire => std::f32::INFINITY,
        }
    }

    pub fn get_value(&self) -> Option<f32> {
		use std::f32::consts::PI;
        match self.frequency {
            Some(f) => {
                match self.component_type {
                    ComponentType::L => {
                        Some(self.reactance / (2.0 * PI * f))
                    },
                    ComponentType::C => {
                        Some(-1.0 / (2.0 * PI * f * self.reactance))
                    },
                    ComponentType::Wire => Some(0.0),
                }
            },
            None => None,
        }
    }

    pub fn get_freq(&self) -> Option<f32> {
        self.frequency
    }
}


impl std::fmt::Display for ReactiveComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ans;

        match self.component_type {
            ComponentType::L => ans = "Inductor:",
            ComponentType::C => ans = "Capacitor",
            ComponentType::Wire => ans = "Wire", 
        }

        let mut ans = format!("{}\n    X = {} Ω ⇔ B = {} mS", 
            ans, 
            self.reactance,
            self.get_susceptance());

        match self.frequency {
            Some(f) => {
                ans = format!("{}\n    {:?} = {} H (@ {} Hz)", ans, self.component_type, self.get_value().unwrap(), f);
            }, 
            None => { },
        }
        write!(f, "{}", ans)
    }
}
