use dim::MapUnsafe;
// use quantiphy_rs::Quantity;
use dimensioned as dim;
use std::{f64::consts::PI, fmt::format};

#[derive(Debug)]
pub enum Component {
    L(dim::si::Henry<f64>),
    C(dim::si::Farad<f64>),
    Wire
}

#[derive(Debug)]
pub struct ReactiveComponent {
    reactance: dim::si::Ohm<f64>,
    frequency: Option<dim::si::Hertz<f64>>,
}

impl ReactiveComponent {
    
    pub fn new(reactance: dim::si::Ohm<f64>) -> Self {
        Self {
            reactance,
            frequency: None,
        }
    }

    fn _get_component_type(&self) -> Component {
        match self.reactance {
            r if r > 0.0 * dim::si::OHM => Component::L(std::f64::NAN * dim::si::H),
            r if r < 0.0 * dim::si::OHM => Component::C(std::f64::NAN * dim::si::F),
            _ => Component::Wire,
        }
    }

    pub fn get_reactance(&self) -> dim::si::Ohm<f64> {
        self.reactance
    }
    
    pub fn get_susceptance(&self) -> dim::si::Siemens<f64> {
        // Return equivalent susceptance value
        match self._get_component_type() {
            Component::L(_) | Component::C(_) => {
                -1.0 / self.reactance
            },
            Component::Wire => {
                std::f64::INFINITY * dim::si::SIE
            },
        }
    }


    fn _get_inductance_value(&self, frequency: dim::si::Hertz<f64>) -> dim::si::Henry<f64> {
        self.reactance / (2.0 * PI * frequency)
        // 232.123 * dim::si::HZ
    }
    
    fn _get_capacitance_value(&self, frequency: dim::si::Hertz<f64>) -> dim::si::Farad<f64> {
        -1.0 / (2.0 * PI * frequency * self.reactance)
    }

    fn _resolve(&self) -> f64 {
        unimplemented!()
    }

    pub fn at(&mut self, frequency: dim::si::Hertz<f64>) -> Component {
        self.frequency = Some(frequency);

        match self._get_component_type() {
            Component::L(_) => {
                Component::L(self._get_inductance_value(frequency))
            },
            Component::C(_) => {
                Component::C(self._get_capacitance_value(frequency))
            },
            Component::Wire => {
                panic!("Panic message");
            },
        }
    }

    fn _get_component_type_str(&self) -> &str {
        match self._get_component_type() {
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
            self._get_component_type_str(), 
            self.reactance,
            self.get_susceptance(),
        );

        // if let Some(f) = self.frequency {
        //     ans = format!(
        //         "{}\n    {:?} = {:?} H (@ {:.5?} Hz)", 
        //         ans, 
        //         self._get_component_type(), 
        //         self.at().unwrap(),
        //         f,
        //     );
        // }
    
        write!(f, "{}", ans)
    }
}


impl std::fmt::Display for Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let mut ans;
        
        ans = format!("    {}  (@ {})",
            match self {
                Component::L(val) => format!("L = {:?}", val),
                Component::C(val) => format!("C = {:?}", val),
                Component::Wire => "".to_string(),
            },
            
        );
    
        write!(f, "{}", ans)
    }
}