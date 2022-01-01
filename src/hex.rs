use hexgrid::{Coordinate, Spacing};
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use std::collections::HashMap;
use std::cmp::{max,min};

pub struct World {
    pub map: HashMap<Coordinate, Type>,
    pub size: f32,
    pub spacing: Spacing,
    pub radius:i32,
}

impl World {
    pub fn new() -> Self {
        let mut map = HashMap::default();
        let mut small_rng = SmallRng::from_entropy();
        let map_radius:i32 =12;

        for q in -map_radius..map_radius {
            let r1 = max(-map_radius, -q - map_radius);
            let r2 = min(map_radius, -q + map_radius);
            for r in r1..r2 {
                let coordinate = Coordinate::from_cubic(q,r);
                let data = if small_rng.gen_bool(0.5) {
                    Type::Red
                } else {
                    Type::Blue
                };
                map.insert(coordinate, data);
                
            }
        }

        let size=28.0;
        
        World { 
            map,
            size,
            spacing: Spacing::FlatTop(size),
            radius:map_radius,
        }
    }
}

pub enum Type {
    Red,
    Blue,
}