pub mod utils;
pub mod maths;
pub mod tests;
use crate::maths::*;


use wasm_bindgen::prelude::*;
use std::collections::HashMap;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}


// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub(crate) use log;


extern crate web_sys;

pub fn now() -> f64 {
    web_sys::window()
        .expect("should have a Window")
        .performance()
        .expect("should have a Performance")
        .now()
}


#[wasm_bindgen]
pub fn greet() {
    log!("Hello, zoop!");
}


#[allow(non_camel_case_types)]
type uuid = u32;


#[wasm_bindgen]
pub struct Machine {
    // uuid
    u: uuid,
    // index
    i: usize,
    // previous position
    pp: Vector,
    // position
    p: Vector,
    // new position
    np: Vector,
    // diameter
    d: float,
    // mass
    m: float,
}


// pub struct TransformerKind {
//     // uuid
//     u: uuid,
//     // label
//     l: String,
//     // text_id
//     t: String,
//     // inputs: map(ResourceKind.uuid, quantity)
//     inputs: HashMap<uuid, float>,
//     // outputs: map(ResourceKind.uuid, quantity)
//     outputs: HashMap<uuid, float>
// }


// pub struct Transformer {
//     // uuid
//     u: uuid,
//     // index
//     i: usize,
// }



#[wasm_bindgen]
pub struct ResourceKind {
    // uuid
    u: uuid,
    // label
    l: String,
    // text_id
    t: String,
    // color
    c: String,
}


#[wasm_bindgen]
pub struct Resource {
    // uuid
    u: uuid,
    // index
    i: usize,
    // position
    p: Vector,
    // diameter
    d: float,
    // kind
    k: uuid,
}


#[wasm_bindgen]
pub struct Universe {
    machines: Vec<Machine>,
    active_machines: HashMap<uuid, usize>,
    inactive_machines: HashMap<uuid, usize>,
    next_uuid: uuid,
    resources: Vec<Resource>,
    active_resources: HashMap<uuid, usize>,
    inactive_resources: HashMap<uuid, usize>,
    active_resources_by_kind: HashMap<uuid, HashMap<uuid, usize> >,
    resource_kinds: HashMap<uuid, ResourceKind>,
    resource_kinds_by_text_id: HashMap<String, uuid>,
    DIAMETER: float,
    CELLS_COUNT_BY_SIDE: usize,
    CELLS_COUNT: usize,
    cells: Vec<Cell>
}


#[wasm_bindgen]
#[derive(Debug)]
pub struct Cell {
    active_resources_by_kind: HashMap<uuid, HashMap<uuid, usize> >,
    active_machines: HashMap<uuid, usize>,
    c9s: Vec<usize>,
    cell_id: usize,
}


#[wasm_bindgen]
pub struct AddMachine {
    position: Vector,
    speed: Vector,
}


#[wasm_bindgen]
impl AddMachine {
    pub fn new(x: float, y: float, dx: float, dy: float) -> AddMachine {
        AddMachine{
            position: Vector{
                x: x,
                y: y,
            },
            speed: Vector{
                x: dx,
                y: dy,
            }
        }
    }
}


#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let CELLS_COUNT_BY_SIDE = 25;
        let mut universe = Universe {
             machines: Vec::new(),
             inactive_machines: HashMap::new(),
             active_machines: HashMap::new(),
             next_uuid: 0,
             resources:  Vec::new(),
             active_resources: HashMap::new(),
             active_resources_by_kind: HashMap::new(),
             inactive_resources: HashMap::new(),
             resource_kinds: HashMap::new(),
             resource_kinds_by_text_id: HashMap::new(),
             DIAMETER: 0.01,
             CELLS_COUNT_BY_SIDE: CELLS_COUNT_BY_SIDE,
             CELLS_COUNT: CELLS_COUNT_BY_SIDE*CELLS_COUNT_BY_SIDE,
             cells: Vec::new(),
        };
        for cell_id in 0..universe.CELLS_COUNT {
            universe.cells.push(Cell{
                active_resources_by_kind: HashMap::new(),
                active_machines: HashMap::new(),
                c9s: cell_ids_9_3(cell_id, universe.CELLS_COUNT_BY_SIDE),
                cell_id: cell_id,
            })
        };
        universe
    }
}


#[wasm_bindgen]
impl Universe {
    pub fn machines(&self) -> *const Machine {
        self.machines.as_ptr()
    }


    pub fn new_uuid(&mut self) -> uuid {
        let u = self.next_uuid;
        self.next_uuid += 1;
        u
    }


    pub fn add_machine_2(&mut self, x: float, y: float, dx: float, dy: float) -> uuid {
        self.add_machine(&AddMachine::new(x,y,dx,dy))
    }
}


#[wasm_bindgen]
impl Universe {
    pub fn add_machine(&mut self, args: & AddMachine) -> uuid {
        let u = self.new_uuid();
        let i = self.machines.len();
        self.machines.push(Machine{
            u: u,
            i: i,
            pp: &args.position - &args.speed,
            p: args.position,
            np: args.position,
            d: self.DIAMETER,
            m: 1.0,
        });
        self.active_machines.insert(u, i);
        u
    }


    pub fn delete_machine(&mut self, u: uuid) {
        let i = self.active_machines[&u];
        self.active_machines.remove(&u);
        self.inactive_machines.insert(u, i);
    }
}


#[wasm_bindgen]
impl Universe {
    pub fn add_resource(
        &mut self,
        kind: uuid,
        x: float,
        y: float,
    ) -> uuid {
        let u = self.new_uuid();
        let i = self.resources.len();
        self.resources.push(Resource{
            u: u,
            i: i,
            p: Vector{
                x:x,
                y:y
            },
            d: self.DIAMETER,
            k: kind,
        });
        self.active_resources.insert(u, i);
        self.active_resources_by_kind.get_mut(&kind).unwrap().insert(u, i);
        u
    }


    pub fn delete_resource(&mut self, u: uuid) {
        let i = self.active_resources[&u];
        self.active_resources_by_kind.get_mut(&self.resources[i].k).unwrap().remove(&u);
        self.active_resources.remove(&u);
        self.inactive_resources.insert(u, i);
    }
}


#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        for i in self.active_machines.values() {
            let mut m1 = &mut self.machines[*i];
            m1.np = &m1.p + &(&m1.p - &m1.pp);
        }
        for i in self.active_machines.values() {
            let mut m1 = &mut self.machines[*i];
            m1.pp = m1.p;
            m1.p = m1.np;
        }
    }

}


#[wasm_bindgen]
impl Universe {
    pub fn reset(&mut self) {
        *self = Universe::new();
    }


    pub fn add_resource_kind (&mut self, text_id: &str, label: &str, color: &str) -> uuid{
        let u = self.new_uuid();
        self.resource_kinds.insert(u, ResourceKind {
            u: u,
            l: label.to_string(),
            t: text_id.to_string(),
            c: color.to_string(),
        });
        self.active_resources_by_kind.insert(u, HashMap::new());
        for i in 0..self.CELLS_COUNT {
            self.cells[i].active_resources_by_kind.insert(u, HashMap::new());
        }
        u
    }
}


pub fn cell_id(p: Vector, s: usize) -> usize {
    (p.x * s as float).floor() as usize
        + ( (p.y * s as float).floor() as usize ) * s
}


pub fn cell_ids_9_3(cell_id: usize, s: usize) -> Vec<usize> {
    cell_ids_9_2(cell_id % s, cell_id / s, s)
}


pub fn cell_ids_9_2(x_: usize, y_: usize, s: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let x = x_ as i32;
    let y = y_ as i32;
    let x_min = 0.max(x-1);
    let y_min = 0.max(y-1);
    let x_max = (s as i32 -1).min(x+1);
    let y_max = (s as i32 -1).min(y+1);
    for j in y_min..y_max+1 {
        for i in x_min..x_max+1 {
            v.push( i as usize + j as usize * s) ;
        }
    }
    v
}


pub fn cell_ids_9(p: Vector, s: usize) -> Vec<usize> {
    cell_ids_9_2((p.x * s as float).floor() as usize, (p.y * s as float).floor() as usize, s)
}


#[wasm_bindgen]
impl Universe {
    pub fn udpate_cells (&mut self) {
        for i in 0..self.CELLS_COUNT {
            for (_, v) in self.cells[i].active_resources_by_kind.iter_mut() {
                v.clear();
            }
            self.cells[i].active_machines.clear();
        }
        for (u, i) in self.active_machines.iter() {
            let cell_id_ = cell_id(self.machines[*i].p, self.CELLS_COUNT_BY_SIDE);
            self.cells[cell_id_].active_machines.insert(*u, *i);
        }
        for (u, i) in self.active_resources.iter() {
            let r = &self.resources[*i];
            let cell_id_ = cell_id(r.p, self.CELLS_COUNT_BY_SIDE);
            self.cells[cell_id_].active_resources_by_kind.get_mut(&r.k).unwrap().insert(*u, *i);
        }
    }
}


#[wasm_bindgen]
impl Universe {
    pub fn closest_resource(& self, machine_u: uuid, kind_u: uuid) -> Option<uuid> {
        let machine = &self.machines[self.active_machines[&machine_u]];
        let mut r = None;
        let mut d_sqrd_min = f32::INFINITY;
        for i in self.active_resources_by_kind[&kind_u].values() {
            let resource = &self.resources[*i];
            let d_sqrd = distance_squared(&machine.p, &resource.p);
            if d_sqrd < d_sqrd_min {
                d_sqrd_min = d_sqrd;
                r = Some(resource.u);
            }
        }
        r
    }


    pub fn closest_resource_2(& self, machine_u: uuid, kind_u: uuid) -> Option<uuid> {
        let machine = &self.machines[self.active_machines[&machine_u]];
        let cell_id = cell_id(machine.p, self.CELLS_COUNT_BY_SIDE);
        let c9s = &self.cells[cell_id].c9s;
        let mut r = None;
        let mut d_sqrd_min = f32::INFINITY;
        for cell_id in c9s {
            let cell = &self.cells[*cell_id];
            for i in cell.active_resources_by_kind[&kind_u].values() {
                let resource = &self.resources[*i];
                let d_sqrd = distance_squared(&machine.p, &resource.p);
                if d_sqrd < d_sqrd_min {
                    d_sqrd_min = d_sqrd;
                    r = Some(resource.u);
                }
            }
        }
        r
    }

}
