use amethyst::ecs::prelude::{Component, DenseVecStorage};
use std::f32::consts::PI;

use crate::components::Destination;

#[derive(Debug, Copy, Clone)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn get_displacement(self) -> f32 {
        let c: f32 = self.x.powi(2) + self.y.powi(2);
        return c.sqrt();
    }

    pub fn set_displacement(self, displacement: f32) -> Velocity {
        let heading = (self.y).atan2(self.x);
        Velocity {
            x: displacement * heading.cos(),
            y: displacement * heading.sin(),
        }
    }

    pub fn turn(self, point_a: Destination, point_b: Destination) -> Velocity {
        let displacement = self.get_displacement();
        let heading = (point_b.y - point_a.y).atan2(point_b.x - point_a.x) * 180.0 / PI;
        Velocity {
            x: displacement * heading.cos(),
            y: displacement * heading.sin(),
        }
    }
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}
