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


impl ops::Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, p2: Vector) -> Vector {
        Vector {
            x: self.x + p2.x,
            y: self.y + p2.y,
        }
    }
}


impl ops::Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, p2: Vector) -> Vector {
        Vector {
            x: self.x - p2.x,
            y: self.y - p2.y,
        }
    }
}


impl ops::Mul<f32> for Vector {
    type Output = Vector;
    fn mul(self, f: float) -> Vector {
        Vector {
            x: self.x * f,
            y: self.y * f,
        }
    }
}


pub fn distance_squared(a: &Vector, b: &Vector) -> float {
    let d = a - b;
    return d.x*d.x + d.y*d.y;
}


pub fn normalize(p: &Vector) -> Vector {
    let d = (p.x * p.x + p.y * p.y).sqrt();
    return Vector {
        x: p.x / d,
        y: p.y / d,
    };
}


pub fn dot(p1: &Vector, p2: &Vector) -> float {
    return p1.x * p2.x + p1.y * p2.y;
}


pub fn rotate(p1: &Vector, p2: &Vector, angle: float) -> Vector {
    // Rotates p2 around p1
    let angle_ = angle * std::f32::consts::PI * 2.0;
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    let cos_ = angle_.cos();
    let sin_ = angle_.sin();
    Vector {
        x: p1.x + dx * cos_ - dy * sin_,
        y: p1.y + dy * cos_ + dx * sin_,
    }
}
