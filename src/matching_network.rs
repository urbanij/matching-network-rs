use num::complex::Complex;
use dimensioned as dim;
use dim::si::*;

use dim::Sqrt;

#[derive(Debug, Clone, Copy, PartialEq)]
enum MatchingNetworkConfig {
    ShuntSeries,
    SeriesShunt,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Solution {
    sol_type: MatchingNetworkConfig,
    shunt_elem: Ohm<f64>,
    series_elem: Ohm<f64>,
}


#[derive(Debug, Clone)]
pub struct MatchingNetwork {
    z1: Complex<Ohm<f64>>,
    z2: Complex<Ohm<f64>>,
    solutions: Vec<Solution>,
}

impl MatchingNetwork {
    pub fn new(
        z1: Complex<Ohm<f64>>,
        z2: Complex<Ohm<f64>>,
    ) -> MatchingNetwork {
        MatchingNetwork {
            z1,
            z2,
            solutions: vec![],
        }
    }

    pub fn get_solutions(&self) -> Vec<Solution> {
        self.solutions.clone()
    }

    pub fn eval_at(&self, freq: dim::si::Hertz<f64>) -> Vec<f64> {
        todo!()
    }

    pub fn solve(&mut self) -> Self {

        let mut solutions = vec![];
        
        let (r1, x1) = (self.z1.re, self.z1.im);
        let (r2, x2) = (self.z2.re, self.z2.im);

        if r1*(r1 - r2) + (x1*x1) >= 0.0 * dim::si::OHM * dim::si::OHM {
            // shunt - series configuration (down coversion)
            
            let x_shu_1 = (r1*x2 + r2*x1 - r1*(x2 - ((r2*(r1*r1 - r2*r1 + x1*x1))/r1).sqrt()))/(r1 - r2);
            let x_shu_2 = (r1*x2 + r2*x1 - r1*(x2 + ((r2*(r1*r1 - r2*r1 + x1*x1))/r1).sqrt()))/(r1 - r2);

            let x_ser_1 = x2 - ((r2*(r1*r1 - r2*r1 + x1*x1))/r1).sqrt();
            let x_ser_2 = x2 + ((r2*(r1*r1 - r2*r1 + x1*x1))/r1).sqrt();
            
            let sol1 = Solution {
                sol_type: MatchingNetworkConfig::ShuntSeries,
                shunt_elem: x_shu_1,
                series_elem: x_ser_1,
            };

            let sol2 = Solution {
                sol_type: MatchingNetworkConfig::ShuntSeries,
                shunt_elem: x_shu_2,
                series_elem: x_ser_2,
            };

            solutions.push(sol1);
            if sol1 != sol2 {
                solutions.push(sol2);
            }
        }


        if r2*(r2 - r1) + x2*x2 >= 0.0 * dim::si::OHM * dim::si::OHM {
            // series - shunt configuration (up coversion)

            let x_shu_1 =  (r1*x2 + (r1*r2*(r2*r2 - r1*r2 + x2*x2)).sqrt())/(r1 - r2);
            let x_shu_2 =  (r1*x2 - (r1*r2*(r2*r2 - r1*r2 + x2*x2)).sqrt())/(r1 - r2);
            
            let x_ser_1 = -(r2*x1 - (r1*r2*(r2*r2 - r1*r2 + x2*x2)).sqrt())/r2;
            let x_ser_2 = -(r2*x1 + (r1*r2*(r2*r2 - r1*r2 + x2*x2)).sqrt())/r2;

            let sol1 = Solution {
                sol_type: MatchingNetworkConfig::SeriesShunt,
                shunt_elem: x_shu_1,
                series_elem: x_ser_1,
            };

            let sol2 = Solution {
                sol_type: MatchingNetworkConfig::SeriesShunt,
                shunt_elem: x_shu_2,
                series_elem: x_ser_2,
            };

        
            solutions.push(sol1);
            if sol1 != sol2 {
                solutions.push(sol2);
            }
        }
        
        MatchingNetwork {
            z1: self.z1,
            z2: self.z2,
            solutions,
        }
    }
}
