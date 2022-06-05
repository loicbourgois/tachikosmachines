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
        for _ in 0..1 {
            log!("\n\n");
            log!("1--------");
            self.test_1();
            log!("2--------");
            self.test_2();
            log!("3--------");
            self.test_3();
            log!("4--------");
            self.test_4();
            log!("5--------");
            self.test_5();
            log!("6--------");
            self.test_6();
            log!("7--------");
            self.test_7();
        }
        self.reset();
    }
}



pub fn almost_eq(a: float, b: float) -> bool {
    (a-b).abs() < 0.01
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
        log!("{:?}", self.machines[self.active_machines[&u]].p.x);
        assert!(almost_eq(self.machines[self.active_machines[&u]].p.x, 0.6));
        assert!(almost_eq(self.machines[self.active_machines[&u]].op.x, 0.5));
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
            let closest_forest_3 = self.closest_resource_all_c9s(*machine_u);
            assert!(closest_forest == closest_forest_2);
            assert!(closest_forest == closest_forest_3);
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


#[wasm_bindgen]
impl Universe {
    pub fn test_5(&mut self) {
        let start = now();
        let forest_kind = self.add_resource_kind(
            "tachicosmachines.forest",
            "Forest",
            "#0F0",
        );
        let resources_count = 10_001;
        let machines_count = 10_000;
        for _ in 0..resources_count {
            self.add_resource(forest_kind,  js_sys::Math::random() as float, js_sys::Math::random() as float);
        }
        for _ in 0..machines_count {
            let machine = self.add_machine(&AddMachine{
                position: Vector{
                    x: js_sys::Math::random() as float,
                    y: js_sys::Math::random() as float,
                },
                speed: Vector{
                    x: 0.0,
                    y: 0.0,
                }
            });
        }
        assert!(self.available_resources_by_kind[&forest_kind].len() == 10_001);
        let start = now();
        self.tick();
        log!("Tick: {:?} ms", now() - start);
        log!("available_resources_by_kind: {:?}", self.available_resources_by_kind[&forest_kind].len());
        assert!(self.available_resources_by_kind[&forest_kind].len() == 1);
        let start = now();
        self.tick();
        log!("Tick: {:?} ms", now() - start);
        self.reset();
    }
}


#[wasm_bindgen]
impl Universe {
    pub fn test_6(&mut self) {
        let start = now();
        let forest_kind = self.add_resource_kind(
            "tachicosmachines.forest",
            "Forest",
            "#0F0",
        );
        self.add_resource(forest_kind,  0.2, 0.2);
        self.add_resource(forest_kind,  0.2, 0.3);
        let machine = self.add_machine(&AddMachine{
            position: Vector {
                x: 0.1,
                y: 0.1,
            },
            speed: Vector {
                x: 0.0,
                y: 0.0,
            }
        });
        self.tick();
        assert!(  self.machines[0].t == Some(0)  );
        self.tick();
        assert!(  self.machines[0].t == Some(0)  );
        self.machines[0].p.x = 0.199;
        self.machines[0].p.y = 0.199;
        self.machines[0].op.x = 0.199;
        self.machines[0].op.y = 0.199;
        self.tick();
        assert!(  self.machines[0].t == Some(1)  );
        self.reset();
    }
}


#[wasm_bindgen]
impl Universe {
    pub fn test_7(&mut self) {
        let forest_kind = self.add_resource_kind(
            "tachicosmachines.forest",
            "Forest",
            "#0F0",
        );
        let resources_count = 1_000;
        let machines_count = 100;
        for _ in 0..resources_count {
            self.add_resource(forest_kind,  js_sys::Math::random() as float, js_sys::Math::random() as float);
        }
        for _ in 0..machines_count {
            let machine = self.add_machine(&AddMachine{
                position: Vector{
                    x: js_sys::Math::random() as float,
                    y: js_sys::Math::random() as float,
                },
                speed: Vector{
                    x: 0.0,
                    y: 0.0,
                }
            });
        }
        for _ in 0..1000 {
            self.tick();
        }
        self.reset();
    }
}
