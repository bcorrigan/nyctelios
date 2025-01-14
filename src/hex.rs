use hexgrid::{Coordinate, Direction, Spacing, Spin};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::cmp::{max, min};
use std::collections::HashMap;

pub struct World {
    pub map: HashMap<Coordinate, Type>,
    pub edge_lookup: HashMap<Coordinate, Coordinate>,
    pub size: f32,
    pub spacing: Spacing,
    pub radius: i32,
    pub rule: Rule,
}

pub struct Rule {
    pub survival: Vec<u8>,
    pub birth: Vec<u8>,
    pub states: u8,
}

impl World {
    pub fn new() -> Self {
        let mut map = HashMap::default();
        let mut small_rng = SmallRng::from_entropy();
        let map_radius: i32 = 20;

        let rule = Rule {
            survival: vec![2, 1],
            birth: vec![2],
            states: 3,
        };

        //coordinates of 6 mirrored hexagonal origins
        let mirror_coords = [
            Coordinate::from_cubic(-map_radius, -map_radius - 1, 2 * map_radius + 1),
            Coordinate::from_cubic(map_radius + 1, -2 * map_radius - 1, map_radius),
            Coordinate::from_cubic(2 * map_radius + 1, -map_radius, -map_radius - 1),
            Coordinate::from_cubic(map_radius, map_radius + 1, -2 * map_radius - 1),
            Coordinate::from_cubic(-map_radius - 1, 2 * map_radius + 1, -map_radius),
            Coordinate::from_cubic(-2 * map_radius - 1, map_radius, map_radius + 1),
        ];

        for q in -map_radius..=map_radius {
            let r1 = max(-map_radius, -q - map_radius);
            let r2 = min(map_radius, -q + map_radius);
            for r in r1..=r2 {
                let coordinate = Coordinate::from_cubic(q, r, 0 - q - r);
                let data = if small_rng.gen_bool(0.5) {
                    Type::On(rule.states - 1)
                } else {
                    Type::Off
                };
                map.insert(coordinate, data);
            }
        }

        let mut edge_lookup: HashMap<Coordinate, Coordinate> = HashMap::default();
        let origin = Coordinate::from_cubic(0, 0, 0);
        for ring_c in origin.ring_iter(map_radius + 1, Spin::CCW(Direction::XY)) {
            for mirror in mirror_coords {
                if ring_c.distance(mirror) <= map_radius {
                    edge_lookup.insert(ring_c, ring_c - mirror);
                    break;
                }
            }
        }

        let size = 7.0;

        World {
            map,
            edge_lookup,
            size,
            spacing: Spacing::FlatTop(size),
            radius: map_radius,
            rule,
        }
    }

    pub fn iterate(&mut self) {
        let mut map: HashMap<Coordinate, Type> = HashMap::default();

        for coord in self.map.keys() {
            let mut raw_nbrs = coord.neighbors();
            let neighbours = raw_nbrs
                .iter_mut()
                .map(|coord| {
                    if self.edge_lookup.contains_key(coord) {
                        self.edge_lookup.get(coord).unwrap()
                    } else {
                        coord
                    }
                })
                .collect::<Vec<_>>();

            let alive_count: u8 =
                neighbours
                    .iter()
                    .fold(0, |acc, coord| match self.map.get(*coord).unwrap() {
                        Type::On(_) => acc + 1,
                        Type::Off => acc,
                    });

            match self.map.get(coord).unwrap() {
                Type::On(s) => {
                    //already On, so we check if it survives
                    if self.rule.survival.contains(&alive_count) {
                        //do we reset to state 5 here?? or just increment existing?
                        let init_s = if s < &(self.rule.states - 1) {
                            s + 1
                        } else {
                            *s
                        };
                        map.insert(*coord, Type::On(init_s));
                    } else {
                        //die = decrement counter
                        let new_s = s - 1;
                        if new_s == 0 {
                            //Off
                            map.insert(*coord, Type::Off);
                        } else {
                            map.insert(*coord, Type::On(new_s));
                        }
                    }
                }
                Type::Off => {
                    if self.rule.birth.contains(&alive_count) {
                        //does this init at s=5 or s=1?
                        map.insert(*coord, Type::On(1));
                    }
                }
            }
        }

        self.map.extend(map.into_iter());
    }
}

pub enum Type {
    On(u8),
    Off,
}
