
// use quantiphy_rs::Quantity;
use dimensioned as dim;
use std::{f64::consts::PI};

#[derive(Debug)]
pub enum Component {
    L(Option<dim::si::Henry<f64>>),
    C(Option<dim::si::Farad<f64>>),
    Wire
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReactiveComponent {
    reactance: dim::si::Ohm<f64>,
    frequency: Option<dim::si::Hertz<f64>>,
}

impl ReactiveComponent {
    
    pub fn new(reactance: dim::si::Ohm<f64>, frequency: Option<dim::si::Hertz<f64>>) -> Self {
        Self {
            reactance,
            frequency,
        }
    }

    pub fn get_reactance(&self) -> dim::si::Ohm<f64> {
        self.reactance
    }

    pub fn get_frequency(&self) -> Option<dim::si::Hertz<f64>> {
        self.frequency
    }
    
    /// Return equivalent susceptance value [Siemens] i.e. -1/reactance
    pub fn get_susceptance(&self) -> dim::si::Siemens<f64> {
        // Return equivalent susceptance value
        match self._get_component() {
            Component::L(_) | Component::C(_) => {
                -1.0 / self.reactance
            },
            Component::Wire => {
                std::f64::INFINITY * dim::si::SIE
            },
        }
    }


    /************ private methods ************/

    /// Given a frequency, resolve inductance [Henry]
    fn _get_inductance_value(&self) -> Option<dim::si::Henry<f64>> {
        Some(self.reactance / (2.0 * PI * self.frequency.unwrap()))
    }
    
    /// Given a frequency, resolve capacitance [Farad]
    fn _get_capacitance_value(&self) -> Option<dim::si::Farad<f64>> {
        Some(-1.0 / (2.0 * PI * self.frequency.unwrap() * self.reactance))
    }

    fn _get_component(&self) -> Component {
        match self.get_reactance() {
            r if r > 0.0 * dim::si::OHM => {
                Component::L(self._get_inductance_value())
            },
            r if r < 0.0 * dim::si::OHM => {
                Component::C(self._get_capacitance_value())
            },
            _ => Component::Wire,
        }
    }

    fn _get_component_str(&self) -> &str {
        match self._get_component() {
            Component::L(_) => "Inductor",
            Component::C(_) => "Capacitor",
            Component::Wire => "Wire", 
        }
    }

}


impl std::fmt::Display for ReactiveComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let mut ans;
    
        ans = format!(
            "{}:\n    X = {:.5} ⇔ B = {:.5}",  
            self._get_component_str(), 
            self.get_reactance(),
            self.get_susceptance(),
        );

        if let Some(freq) = self.frequency {
            ans = format!(
                "{}\n    {} (@ {:.5?})", 
                ans, 
                self._get_component(),
                freq,
            );
        }
    
        write!(f, "{}", ans)
    }
}



impl std::fmt::Display for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        let ans = match self {
            Component::L(val) => {
                match val {
                    Some(v) => format!("L = {}", v),
                    None => "L".to_string(),
                }
            },
            Component::C(val) => {
                match val {
                    Some(v) => format!("C = {}", v),
                    None => "C".to_string(),
                }
            },
            Component::Wire => "".to_string(),
        };

        write!(f, "{}", ans)
    }
}
