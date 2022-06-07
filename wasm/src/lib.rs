pub mod utils;
pub mod maths;
pub mod tests;
pub mod cell;
use crate::cell::*;
use crate::maths::*;


use wasm_bindgen::prelude::*;
use std::collections::HashMap;


extern crate web_sys;


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


pub fn now() -> f64 {
    web_sys::window()
        .expect("should have a Window")
        .performance()
        .expect("should have a Performance")
        .now()
}


#[wasm_bindgen]
pub fn greet() {
    log!("Hello!");
}


#[allow(non_camel_case_types)]
type uuid = u32;


#[wasm_bindgen]
#[derive(Debug)]
#[repr(C)]
pub struct Machine {
    // uuid
    u: uuid,
    // index
    i: usize,
    // previous position
    op: Vector,
    // position
    p: Vector,
    // new position
    pn: Vector,
    // diameter
    d: float,
    // mass
    m: float,
    // target
    t: Option<usize>,
    // has target
    ht: u32,
    //
    store: HashMap<uuid, float>,
}


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
    growth_rate: float,
    split_cost: Option<float>,
    // growth_precursors:
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
    // active
    a: u32,
    //
    store: float,
}

type Cells = Vec<Cell>;
type Resources = Vec<Resource>;


#[wasm_bindgen]
pub struct Universe {
    machines: Vec<Machine>,
    active_machines: HashMap<uuid, usize>,
    inactive_machines: HashMap<uuid, usize>,
    next_uuid: uuid,
    resources: Resources,
    active_resources: HashMap<uuid, usize>,
    inactive_resources: HashMap<uuid, usize>,
    active_resources_by_kind: HashMap<uuid, HashMap<uuid, usize> >,
    available_resources_by_kind: HashMap<uuid, HashMap<uuid, usize> >,
    resource_kinds: HashMap<uuid, ResourceKind>,
    resource_kinds_by_text_id: HashMap<String, uuid>,
    base_diameter: float,
    CELLS_COUNT_BY_SIDE: usize,
    CELLS_COUNT: usize,
    cells: Cells,
    step: usize,
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
    pub fn new(base_diameter: float) -> Universe {
        let CELLS_COUNT_BY_SIDE = (0.25 / base_diameter) as usize;
        let mut universe = Universe {
             machines: Vec::new(),
             inactive_machines: HashMap::new(),
             active_machines: HashMap::new(),
             next_uuid: 0,
             resources:  Vec::new(),
             active_resources: HashMap::new(),
             active_resources_by_kind: HashMap::new(),
             inactive_resources: HashMap::new(),
             available_resources_by_kind: HashMap::new(),
             resource_kinds: HashMap::new(),
             resource_kinds_by_text_id: HashMap::new(),
             base_diameter: base_diameter,
             CELLS_COUNT_BY_SIDE: CELLS_COUNT_BY_SIDE,
             CELLS_COUNT: CELLS_COUNT_BY_SIDE * CELLS_COUNT_BY_SIDE,
             cells: Vec::new(),
             step: 0,
        };
        for cell_id in 0..universe.CELLS_COUNT {
            universe.cells.push(Cell{
                available_resources_by_kind: HashMap::new(),
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


    pub fn machines_count(&self) -> usize {
        self.machines.len()
    }


    pub fn resources(&self) -> *const Resource {
        self.resources.as_ptr()
    }


    pub fn resources_count(&self) -> usize {
        self.resources.len()
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
        let mut store = HashMap::with_capacity(self.resource_kinds.len());
        for k in self.resource_kinds.keys() {
            store.insert(*k, 3.25);
        }
        store.shrink_to_fit();
        self.machines.push(Machine{
            u: u,
            i: i,
            op: &args.position - &args.speed,
            p: args.position,
            pn: args.position,
            d: self.base_diameter,
            m: 27.0,
            t: None,
            ht: 0,
            store: store,
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


pub fn closest_available_resource_all_full_scan(
    machine: & Machine,
    resource_kinds: & HashMap<uuid, ResourceKind>,
    available_resources_by_kind: & HashMap<uuid, HashMap<uuid, usize> >,
    resources: & Vec<Resource>
) -> Option<usize> {
    let mut r = None;
    let mut d_sqrd_min = f32::INFINITY;
        for kind_u in resource_kinds.keys() {
            for (u, i) in available_resources_by_kind[&kind_u].iter() {
                let resource = & resources[*i];
                let d_sqrd = distance_squared(&machine.p, &resource.p);
                if d_sqrd < d_sqrd_min {
                    d_sqrd_min = d_sqrd;
                    r = Some(resource.i);
                }
            }
        }
    r
}


pub fn closest_available_resource_all_c9s(
    machine: & Machine,
    resource_kinds: & HashMap<uuid, ResourceKind>,
    available_resources_by_kind: & HashMap<uuid, HashMap<uuid, usize> >,
    resources: & Vec<Resource>,
    CELLS_COUNT_BY_SIDE: usize,
    cells: &Vec<Cell>,
) -> Option<usize> {
    let cell_id = cell_id(machine.p, CELLS_COUNT_BY_SIDE);
    let c9s = &cells[cell_id].c9s;
    let mut r = None;
    let mut d_sqrd_min = f32::INFINITY;
    for cell_id in c9s {
        let cell = &cells[*cell_id];
        for kind_u in resource_kinds.keys() {
            for i in cell.available_resources_by_kind[&kind_u].values() {
                let resource = & resources[*i];
                let d_sqrd = distance_squared(&machine.p, &resource.p);
                if d_sqrd < d_sqrd_min {
                    d_sqrd_min = d_sqrd;
                    r = Some(resource.i);
                }
            }
        }
    }
    r
}


pub fn find_target(
    machine: &mut Machine,
    available_resources_by_kind: &mut HashMap<uuid, HashMap<uuid, usize>>,
    resource_kinds: & HashMap<uuid, ResourceKind>,
    resources: & Vec<Resource>,
    active_resources: & HashMap<uuid, usize>,
    CELLS_COUNT_BY_SIDE: usize,
    cells: &mut Vec<Cell>,
) {
    machine.t = match closest_available_resource_all_c9s(
        machine,
        resource_kinds,
        available_resources_by_kind,
        resources,
        CELLS_COUNT_BY_SIDE,
        cells,
    ) {
        Some(x) => Some(x),
        None => closest_available_resource_all_full_scan(
            machine,
            resource_kinds,
            available_resources_by_kind,
            resources,
        )
    };
    match machine.t {
        Some(r_i) => {
            let ressource = &resources[r_i];
            let kind = ressource.k;
            let cell_id = cell_id(ressource.p, CELLS_COUNT_BY_SIDE);
            cells.get_mut(cell_id).unwrap().available_resources_by_kind.get_mut(&kind).unwrap().remove(&ressource.u);
            available_resources_by_kind.get_mut(&kind).unwrap().remove(&ressource.u);
        }
        None => {}
    }
}


pub fn resource_by_u<'a>(
    resources: &'a Vec<Resource>,
    active_resources: &'a HashMap<uuid, usize>,
    resource_u: uuid,
) -> &'a Resource {
    &resources[active_resources[&resource_u] ]
}


#[wasm_bindgen]
impl Universe {
    pub fn update_targets(&mut self) {
        for i in self.active_machines.values() {
            let m1 = &mut self.machines[*i];
            match m1.t {
                None => {
                    find_target(
                        m1,
                        &mut self.available_resources_by_kind,
                        &self.resource_kinds,
                        &self.resources,
                        &self.active_resources,
                        self.CELLS_COUNT_BY_SIDE,
                        &mut self.cells,
                    )
                },
                Some(_) => {}
            }
            m1.ht = match m1.t {
                Some(_) => 1,
                None => 0,
            }
        }
    }
}


#[wasm_bindgen]
impl Universe {
    pub fn move_machines(&mut self) {
        for i in self.active_machines.values() {
            let mut m1 = &mut self.machines[*i];
            let target_acceleration = match m1.t {
                None =>{
                    Vector{x:0.0, y:0.0}
                },
                Some(r_i) => {
                    normalize(& (self.resources[r_i].p - m1.p) ) * 0.00001
                }
            };
            let deceleration = (&m1.p - &m1.op) * (- 0.01);
            m1.pn = &m1.p
                + &(&m1.p - &m1.op)
                + target_acceleration
                + deceleration;
        }
        for i in self.active_machines.values() {
            let mut m1 = &mut self.machines[*i];
            m1.op = m1.p;
            m1.p = m1.pn;
        }
    }
}


#[wasm_bindgen]
impl Universe {
    pub fn collect_resources(&mut self) {
        let mut resources_to_delete = Vec::new();
        for i in self.active_machines.values() {
            let m1 = &mut self.machines[*i];
            match m1.t {
                None =>{}
                Some(r_i) => {
                    let r = &self.resources[r_i];
                    let r_p = r.p;
                    let r_u = r.u;
                    if distance_squared(&m1.p, &r_p) < self.base_diameter * self.base_diameter {
                        m1.t = None;
                        *m1.store.get_mut( &r.k ).unwrap() += r.store;
                        resources_to_delete.push(r_u);
                    }
                }
            }
        }
        for r_u in resources_to_delete.iter() {
            self.delete_resource(*r_u);
        }
    }
}


pub fn count_resources_at(
    p: Vector,
    diameter: float,
    CELLS_COUNT_BY_SIDE: usize,
    cells: &Cells,
    resources: &Resources,
) -> usize {
    let cell_id = cell_id(p, CELLS_COUNT_BY_SIDE);
    let mut count = 0;
    for cell_id_2 in &cells[cell_id].c9s {
        let cell = &cells[*cell_id_2];
        for (kind_u, active_resources) in cell.active_resources_by_kind.iter() {
            for (i, resource_u) in active_resources {
                let r = &resources[*resource_u];
                let diams = diameter*0.5+r.d*0.5;
                if distance_squared(&r.p, &p) < diams*diams {
                    count+= 1;
                }
            }
        }
    }
    count
}


#[wasm_bindgen]
impl Universe {
    pub fn grow_resources(&mut self) {
        for (u,i) in self.active_resources.clone() {
            let (new_resource_p, r_kind_u) = {
                let r_kind = &self.resource_kinds[&self.resources[i].k];
                self.resources[i].store += r_kind.growth_rate;
                let r1 = &self.resources[i];
                let new_resource_p: Option<Vector> = match (r1.store >= 1.0, r_kind.split_cost) {
                    (true, Some (cost)) => {
                        let new_p = rotate(
                            &r1.p,
                            &(r1.p + Vector{x:r1.d * 1.01, y:0.0}),
                            js_sys::Math::random() as float,
                        );
                        match (count_resources_at(
                            new_p,
                            r1.d,
                            self.CELLS_COUNT_BY_SIDE,
                            &self.cells,
                            &self.resources,
                        ), distance_squared(&new_p, &Vector{x:0.5, y:0.5}) < 0.25 ) {
                            (0, true) => {
                                self.resources[i].store -= cost;
                                Some(new_p)
                            }
                            _ => {
                                None
                            }
                        }
                    }
                    _ => {
                        None
                    }
                };
                let mut r1_mut = &mut self.resources[i];
                r1_mut.store = r1_mut.store.max(0.0).min(1.0);
                (new_resource_p, r_kind.u)
            };
            match new_resource_p {
                Some(p) => {
                    self.add_resource(
                        r_kind_u,
                        p.x,
                        p.y,
                    );
                }
                None => {}
            };

        }
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
        let p = Vector{
            x:x,
            y:y
        };
        let cell_id = cell_id(p, self.CELLS_COUNT_BY_SIDE);
        let i = if self.inactive_resources.len() > 0 {
            let (u_, i_) = self.inactive_resources.iter().next().unwrap();
            let uu = (*u_).clone();
            let i:usize = (*i_).clone();
            self.inactive_resources.remove(&uu);
            self.resources[i] = Resource{
                u: u,
                i: i,
                p: p,
                d: self.base_diameter,
                k: kind,
                a: 1,
                store: 0.0,
            };
            i
        } else {
            let i = self.resources.len();
            self.resources.push(Resource{
                u: u,
                i: i,
                p: p,
                d: self.base_diameter,
                k: kind,
                a: 1,
                store: 0.0,
            });
            i
        };
        self.active_resources.insert(u, i);
        self.active_resources_by_kind.get_mut(&kind).unwrap().insert(u, i);
        self.available_resources_by_kind.get_mut(&kind).unwrap().insert(u, i);
        self.cells.get_mut(cell_id).unwrap().available_resources_by_kind.get_mut(&kind).unwrap().insert(u, i);
        u
    }


    pub fn delete_resource(&mut self, u: uuid) {
        let i = self.active_resources[&u];
        self.resources[i].a = 0;
        let resource = &self.resources[i];
        let cell_id = cell_id(resource.p, self.CELLS_COUNT_BY_SIDE);
        self.active_resources_by_kind.get_mut(&resource.k).unwrap().remove(&u);
        self.available_resources_by_kind.get_mut(&resource.k).unwrap().remove(&u);
        self.cells.get_mut(cell_id).unwrap().available_resources_by_kind.get_mut(&resource.k).unwrap().remove(&u);
        self.active_resources.remove(&u);
        self.inactive_resources.insert(u, i);
    }
}


#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        self.udpate_cells();
        self.move_machines();
        self.grow_resources();
        self.collect_resources();
        self.update_targets();
        self.step += 1;
    }
}


#[wasm_bindgen]
impl Universe {
    pub fn reset(&mut self) {
        *self = Universe::new(self.base_diameter);
    }
}


#[wasm_bindgen]
impl Universe {
    pub fn add_resource_kind (
        &mut self,
        text_id: &str,
        label: &str,
        color: &str,
        growth_rate: float,
    ) -> uuid{
        let u = self.new_uuid();
        self.resource_kinds.insert(u, ResourceKind {
            u: u,
            l: label.to_string(),
            t: text_id.to_string(),
            c: color.to_string(),
            growth_rate: growth_rate,
            split_cost: Some(0.25),
        });
        self.active_resources_by_kind.insert(u, HashMap::new());
        self.available_resources_by_kind.insert(u, HashMap::new());
        for i in 0..self.CELLS_COUNT {
            self.cells[i].active_resources_by_kind.insert(u, HashMap::new());
            self.cells[i].available_resources_by_kind.insert(u, HashMap::new());
        }
        u
    }
}


pub fn cell_id(p: Vector, s: usize) -> usize {
    let x = 0.max( (s-1) .min(  (p.x * s as float).floor() as usize  ));
    let y = 0.max( (s-1) .min(  (p.y * s as float).floor() as usize  ));
    let id = x + y * s;
    id
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


    pub fn closest_resource_all_c9s(& self, machine_u: uuid) -> Option<uuid> {
        let machine = &self.machines[self.active_machines[&machine_u]];
        let cell_id = cell_id(machine.p, self.CELLS_COUNT_BY_SIDE);
        let c9s = &self.cells[cell_id].c9s;
        let mut r = None;
        let mut d_sqrd_min = f32::INFINITY;
        for cell_id in c9s {
            let cell = &self.cells[*cell_id];
            for kind_u in self.resource_kinds.keys() {
                for i in cell.active_resources_by_kind[&kind_u].values() {
                    let resource = &self.resources[*i];
                    let d_sqrd = distance_squared(&machine.p, &resource.p);
                    if d_sqrd < d_sqrd_min {
                        d_sqrd_min = d_sqrd;
                        r = Some(resource.u);
                    }
                }
            }
        }
        r
    }
}
