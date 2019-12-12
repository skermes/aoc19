use itertools::Itertools;

use crate::problem::Problem;

trait Normalizable {
    fn normalize(&self) -> Self;
}

impl Normalizable for isize {
    fn normalize(&self) -> Self {
        if self < &0 {
            -1
        } else if self > &0 {
            1
        } else {
            0
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Z3 {
    x: isize,
    y: isize,
    z: isize
}

impl Z3 {
    fn add(&mut self, other: &Z3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Body {
    position: Z3,
    velocity: Z3
}

impl Body {
    fn from_triple(x: isize, y: isize, z: isize) -> Body {
        Body {
            position: Z3 { x: x, y: y, z: z },
            velocity: Z3 { x: 0, y: 0, z: 0 }
        }
    }

    fn delta_v_from(&self, other: &Body) -> Z3 {
        Z3 {
            x: -(self.position.x - other.position.x).normalize(),
            y: -(self.position.y - other.position.y).normalize(),
            z: -(self.position.z - other.position.z).normalize()
        }
    }

    fn step(&mut self, delta_v: Z3) {
        self.velocity.add(&delta_v);
        self.position.add(&self.velocity);
    }

    fn potential_energy(&self) -> usize {
        (self.position.x.abs() + self.position.y.abs() + self.position.z.abs()) as usize
    }

    fn kinetic_energy(&self) -> usize {
        (self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()) as usize
    }

    fn total_energy(&self) -> usize {
        self.potential_energy() * self.kinetic_energy()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct OrbitalSystem {
    bodies: Vec<Body>,
    step_count: usize
}

impl OrbitalSystem {
    fn from_bodies(bodies: Vec<Body>) -> OrbitalSystem {
        OrbitalSystem {
            bodies: bodies,
            step_count: 0
        }
    }

    fn step(&mut self) {
        let mut delta_vs = self.bodies.iter()
            .map(|_| Z3 { x: 0, y: 0, z: 0 })
            .collect_vec();

        for (i, body) in self.bodies.iter().enumerate() {
            for other in &self.bodies {
                delta_vs.get_mut(i).unwrap().add(&body.delta_v_from(&other));
            }
        }

        for (body, delta_v) in self.bodies.iter_mut().zip(delta_vs) {
            body.step(delta_v);
        }

        self.step_count += 1;
    }

    fn n_steps(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }

    fn total_energy(&self) -> usize {
        self.bodies.iter().map(|b| b.total_energy()).sum()
    }
}

pub struct DayTwelve {}

impl Problem for DayTwelve {
    fn name(&self) -> String {
        "The N-Body Problem".to_string()
    }

    // Raw Input
    // <x=9, y=13, z=-8>
    // <x=-3, y=16, z=-17>
    // <x=-4, y=11, z=-10>
    // <x=0, y=-2, z=-2>

    fn part_one(&self, input: &str) -> String {
        let mut system = OrbitalSystem::from_bodies(vec![
            Body::from_triple(9, 13, -8),
            Body::from_triple(-3, 16, -17),
            Body::from_triple(-4, 11, -10),
            Body::from_triple(0, -2, -2)
        ]);
        system.n_steps(1000);

        system.total_energy().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        format!("{}", "Part two not yet implemented.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq};

    #[test]
    fn example_1() {
        let mut system = OrbitalSystem::from_bodies(vec![
            Body::from_triple(-1, 0, 2),
            Body::from_triple(2, -10, -7),
            Body::from_triple(4, -8, 8),
            Body::from_triple(3, 5, -1)
        ]);

        system.step();

        assert_eq!(OrbitalSystem {
            bodies: vec![
                Body {
                    position: Z3 { x: 2, y: -1, z: 1 },
                    velocity: Z3 { x: 3, y: -1, z: -1 }
                },
                Body {
                    position: Z3 { x: 3, y: -7, z: -4 },
                    velocity: Z3 { x: 1, y: 3, z: 3 }
                },
                Body {
                    position: Z3 { x: 1, y: -7, z: 5 },
                    velocity: Z3 { x: -3, y: 1, z: -3 }
                },
                Body {
                    position: Z3 { x: 2, y: 2, z: 0 },
                    velocity: Z3 { x: -1, y: -3, z: 1 }
                },
            ],
            step_count: 1
        }, system);
    }

    #[test]
    fn example_2() {
        let mut system = OrbitalSystem::from_bodies(vec![
            Body::from_triple(-8, -10, 0),
            Body::from_triple(5, 5, 10),
            Body::from_triple(2, -7, 3),
            Body::from_triple(9, -8, -3)
        ]);

        system.n_steps(100);

        assert_eq!(OrbitalSystem {
            bodies: vec![
                Body {
                    position: Z3 { x: 8, y: -12, z: -9 },
                    velocity: Z3 { x: -7, y: 3, z: 0 }
                },
                Body {
                    position: Z3 { x: 13, y: 16, z: -3 },
                    velocity: Z3 { x: 3, y: -11, z: -5 }
                },
                Body {
                    position: Z3 { x: -29, y: -11, z: -1 },
                    velocity: Z3 { x: -3, y: 7, z: 4 }
                },
                Body {
                    position: Z3 { x: 16, y: -13, z: 23 },
                    velocity: Z3 { x: 7, y: 1, z: 1 }
                }
            ],
            step_count: 100
        }, system);

        assert_eq!(29, system.bodies[0].potential_energy());
        assert_eq!(10, system.bodies[0].kinetic_energy());
        assert_eq!(290, system.bodies[0].total_energy());
        assert_eq!(32, system.bodies[1].potential_energy());
        assert_eq!(19, system.bodies[1].kinetic_energy());
        assert_eq!(608, system.bodies[1].total_energy());
        assert_eq!(41, system.bodies[2].potential_energy());
        assert_eq!(14, system.bodies[2].kinetic_energy());
        assert_eq!(574, system.bodies[2].total_energy());
        assert_eq!(52, system.bodies[3].potential_energy());
        assert_eq!(9, system.bodies[3].kinetic_energy());
        assert_eq!(468, system.bodies[3].total_energy());

        assert_eq!(1940, system.total_energy());
    }
}
