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

    pub fn eval_at(&self, freq: dim::si::Hertz<f64>) -> () {
        println!("# solutions: {}", self.solutions.len());

        for solution in self.solutions.iter() {
            println!("\n{}", 
                match solution.sol_type {
                    MatchingNetworkConfig::SeriesShunt => "series-shunt",
                    MatchingNetworkConfig::ShuntSeries => "shunt-series",
                }
            );

            println!("\tX = {:.5} Ω", solution.shunt_elem.value_unsafe);
            println!("\tX = {:.5} Ω", solution.series_elem.value_unsafe);
        }
    }

    pub fn solve(mut self) -> Self {

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

        self.solutions = solutions;
        
        self

        // MatchingNetwork {
        //     z1: self.z1,
        //     z2: self.z2,
        //     solutions,
        // }
    }
}


impl std::fmt::Display for MatchingNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let mut ans;

        let z1_without_uom = Complex::new(
            self.z1.re.value_unsafe,
            self.z1.im.value_unsafe,
        );
        let z2_without_uom = Complex::new(
            self.z2.re.value_unsafe,
            self.z2.im.value_unsafe,
        );

        ans = format!(
            "{} Ω ==> {} Ω",
            z1_without_uom, // / dim::si::OHM, 
            z2_without_uom, // / dim::si::OHM,
        );

        // if let Some(freq) = self.frequency {
        //     ans = format!(
        //         "{}\n    {} (@ {:.5?} Hz)", 
        //         ans, 
        //         self._get_component(),
        //         freq / dim::si::HZ,
        //     );
        // }
    
        write!(f, "{}", ans)
    }
}
