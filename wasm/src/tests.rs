use wasm_bindgen::prelude::wasm_bindgen;
use crate::Universe;
use crate::Vector;
use crate::AddMachine;
use crate::log;
use crate::now;
use crate::float;


#[wasm_bindgen]
impl Universe {
    pub fn test(&mut self) {
        for _ in 0..3 {
            self.test_1();
            self.test_2();
            self.test_3();
            self.test_4();
        }
        self.reset();
    }
}


#[wasm_bindgen]
impl Universe {
    pub fn test_1(&mut self) {
        let u = self.add_machine(&AddMachine{
            position: Vector{
                x: 0.5,
                y: 0.5,
            },
            speed: Vector{
                x: 0.1,
                y: 0.0,
            }
        });
        self.tick();
        assert!(self.machines[self.active_machines[&u]].p.x == 0.6);
        assert!(self.machines[self.active_machines[&u]].pp.x == 0.5);
        self.delete_machine(u);
        let forest = self.add_resource_kind(
            "tachicosmachines.forest",
            "Forest",
            "#0F0",
        );
        let r_u = self.add_resource(forest, 0.2, 0.3);
        self.delete_resource(r_u);
        self.reset();
    }
}


#[wasm_bindgen]
impl Universe {
    pub fn test_2(&mut self) {
        let machine = self.add_machine(&AddMachine{
            position: Vector{
                x: 0.5,
                y: 0.5,
            },
            speed: Vector{
                x: 0.1,
                y: 0.0,
            }
        });
        let forest_kind = self.add_resource_kind(
            "tachicosmachines.forest",
            "Forest",
            "#0F0",
        );
        let forest_1 = self.add_resource(forest_kind, 0.2, 0.3);
        let forest_2 = self.add_resource(forest_kind, 0.4, 0.4);
        let forest_3 = self.add_resource(forest_kind, 0.45, 0.15);
        let closest_forest = self.closest_resource(machine, forest_kind);
        assert!(closest_forest.unwrap() == forest_2);
        self.reset();
    }
}


#[wasm_bindgen]
impl Universe {
    pub fn test_3(&mut self) {
        let start = now();
        let forest_kind = self.add_resource_kind(
            "tachicosmachines.forest",
            "Forest",
            "#0F0",
        );
        let resources_count = 1000;
        let machines_count = 1000;
        for _ in 0..resources_count {
            self.add_resource(forest_kind,  js_sys::Math::random() as float, js_sys::Math::random() as float);
        }
        for _ in 0..machines_count {
            let machine = self.add_machine(&AddMachine{
                position: Vector{
                    x: 0.5,
                    y: 0.5,
                },
                speed: Vector{
                    x: 0.1,
                    y: 0.0,
                }
            });
        }
        self.udpate_cells();
        log!("Init: {:?} ms", now() - start);
        let start = now();
        for machine_u in self.active_machines.keys() {
            let closest_forest = self.closest_resource(*machine_u, forest_kind);
            let closest_forest_2 = self.closest_resource_2(*machine_u, forest_kind);
            assert!(closest_forest == closest_forest_2);
        }
        log!("Compute:  {:?} ms", now() - start);
        let start = now();
        for machine_u in self.active_machines.keys() {
            let closest_forest_2 = self.closest_resource_2(*machine_u, forest_kind);
        }
        log!("Compute2: {:?} ms", now() - start);
        self.reset();
    }
}


#[wasm_bindgen]
impl Universe {
    pub fn test_4(&mut self) {
        let start = now();
        let forest_kind = self.add_resource_kind(
            "tachicosmachines.forest",
            "Forest",
            "#0F0",
        );
        let resources_count = 10_000;
        let machines_count = 10_000;
        for _ in 0..resources_count {
            self.add_resource(forest_kind,  js_sys::Math::random() as float, js_sys::Math::random() as float);
        }
        for _ in 0..machines_count {
            let machine = self.add_machine(&AddMachine{
                position: Vector{
                    x: 0.5,
                    y: 0.5,
                },
                speed: Vector{
                    x: 0.1,
                    y: 0.0,
                }
            });
        }
        self.udpate_cells();
        log!("Init: {:?} ms", now() - start);
        let start = now();
        for machine_u in self.active_machines.keys() {
            let closest_forest_2 = self.closest_resource_2(*machine_u, forest_kind);
        }
        log!("Compute2: {:?} ms", now() - start);
        self.reset();
    }
}
