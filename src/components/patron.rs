use amethyst::ecs::prelude::{Component, DenseVecStorage};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug)]
pub struct Patron {
    pub satisfaction: u8,
    pub velocity: [f32; 2],
    pub origin: [f32; 2],
}

enum Direction {
    Left,
    Right,
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0, 2) {
            0 => Direction::Left,
            _ => Direction::Right,
        }
    }
}

enum Plane {
    Fg,
    Mid,
    Bg,
}

impl Distribution<Plane> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Plane {
        match rng.gen_range(0, 3) {
            0 => Plane::Fg,
            1 => Plane::Mid,
            _ => Plane::Bg,
        }
    }
}

impl Patron {
    pub fn generate() -> Patron {
        let mut rng = rand::thread_rng();

        let direction: Direction = rand::random();
        let proposed_velocity = [rng.gen_range(15.0, 25.0), 0.0];

        let velocity = match direction {
            Direction::Left => [-proposed_velocity[0], proposed_velocity[1]],
            _ => proposed_velocity,
        };

        let plane: Plane = rand::random();
        let origin_y = match plane {
            Plane::Fg => 12.0,
            Plane::Mid => 24.0,
            Plane::Bg => 36.0,
        };

        let origin = match direction {
            Direction::Right => [-10.0, origin_y],
            _ => [170.0, origin_y],
        };

        Patron {
            satisfaction: rng.gen_range(0, 8),
            velocity,
            origin,
        }
    }
}

impl Component for Patron {
    type Storage = DenseVecStorage<Self>;
}
