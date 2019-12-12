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

        format!("{}", "Part one not yet implemented.")
    }

    fn part_two(&self, input: &str) -> String {
        format!("{}", "Part two not yet implemented.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
