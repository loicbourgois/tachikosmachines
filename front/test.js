import {
  machine_struct_size,
  resource_struct_size,
  keep_drawing,
  keep_ticking,
  MACHINES,
} from "./constants.js"
import {
  machine,
  resource,
} from "./memory.js"
import {
  assert_eq
} from "./utils.js"


const test_1 = (tm, tml) => {
  const universe = tml.Universe.new(
    0.01,
    0.001,
    2.0,
  )
  const forest_k = universe.add_resource_kind(
    "tachicosmachines.forest",
    "Forest",
    "#0F0",
    0.001
  )
  const kind_count = 1;
  console.log("forest_k", forest_k)
  universe.add_machine_2(0.2, 0.3, 0.1, 0.1)
  universe.add_machine_2(0.8, 0.9, 0.1, 0.1)
  universe.add_resource(forest_k, 0.201, 0.301)
  universe.add_resource(forest_k, 0.8, 0.1)
  universe.tick();
  const machines = new DataView(
    tm.memory.buffer,
    universe.machines(),
    universe.machines_count() * 4 * machine_struct_size
  );
  const machines_stores = new DataView(
    tm.memory.buffer,
    universe.machines_stores(),
    universe.machines_stores_count() * 4
  );
  console.log( universe.machines_stores_count() )
  console.log(machine(machines, machines_stores, kind_count, 0))
  console.log(machine(machines, machines_stores, kind_count, 1))
  assert_eq(machine(machines, machines_stores, kind_count, 0).p.x, 0.3)
  assert_eq(machine(machines, machines_stores, kind_count, 1).p.y, 1.0)
  const resources = new DataView(
    tm.memory.buffer,
    universe.resources(),
    universe.resources_count() * 4 * resource_struct_size
  );
  universe.tick();
  assert_eq(resource(resources, 0).p.x, 0.201)
  assert_eq(resource(resources, 1).p.x, 0.8)
  console.log( resource(resources, 0) )
  console.log( resource(resources, 1) )
}


const test = (tm, tml) => {
  test_1(tm, tml)
}


export {
  test,
}
