use wasm_bindgen::prelude::wasm_bindgen;
use std::ops;

#[allow(non_camel_case_types)]
pub type float = f32;

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: float,
    pub y: float,
}


impl ops::Sub<&Vector> for &Vector {
    type Output = Vector;
    fn sub(self, p2: &Vector) -> Vector {
        Vector {
            x: self.x - p2.x,
            y: self.y - p2.y,
        }
    }
}


impl ops::Add<&Vector> for &Vector {
    type Output = Vector;
    fn add(self, p2: &Vector) -> Vector {
        Vector {
            x: self.x + p2.x,
            y: self.y + p2.y,
        }
    }
}


pub fn distance_squared(a: &Vector, b: &Vector) -> float {
    let d = a - b;
    return d.x*d.x + d.y*d.y;
}
