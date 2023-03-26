use crate::map::HexMap;
use hexagon_tiles::hexagon::Hex;
use std::cmp::{max, min};

pub trait Hexagons {
    fn build(&mut self, radius: i32);
}

impl Hexagons for HexMap {
    fn build(&mut self, radius: i32) {
        let n = radius;

        for q in -n..=n {
            let r1 = max(-n, -q - n);
            let r2 = min(n, -q + n);

            for r in r1..=r2 {
                let rep = String::from(format!("({q},{r})"));
                self.map.insert(Hex::new(q, r), rep);
            }
        }
    }
}
