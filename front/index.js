import * as tml from "../wasm/tachikosmachines.js";
import {
  fill_circle,
  line,
  clear,
}
from "./canvas.js"


const tm = await tml.default()
const machine_struct_size = 13
const resource_struct_size = 7
const keep_drawing = true
const keep_ticking = true


const assert_eq = (a, b, message) => {
    if ( Math.abs(a - b) > 0.0000001 ) {
        throw (message || "Assertion failed" ) + `Got ${a}, Expected ${b}`;
    }
}


const machine = (machines, i) => {
  const size = machine_struct_size*4
  return {
    u: machines.getUint32(i*size+4*0, true),
    i: machines.getUint32(i*size+4*1, true),
    op: {
      x: machines.getFloat32(i*size+4*2, true),
      y: machines.getFloat32(i*size+4*3, true),
    },
    p: {
      x: machines.getFloat32(i*size+4*4, true),
      y: machines.getFloat32(i*size+4*5, true),
    },
    pn: {
      x: machines.getFloat32(i*size+4*6, true),
      y: machines.getFloat32(i*size+4*7, true),
    },
    d: machines.getFloat32(i*size+4*8, true),
    m: machines.getFloat32(i*size+4*9, true),
    t: machines.getUint32(i*size+4*11, true),
    ht: machines.getUint32(i*size+4*12, true),
    // m: machines.getFloat32(i*size+4*9, true),
  }
}


const resource = (resources, i) => {
  const size = resource_struct_size*4
  return {
    u: resources.getUint32(i*size+4*0, true),
    i: resources.getUint32(i*size+4*1, true),
    p: {
      x: resources.getFloat32(i*size+4*2, true),
      y: resources.getFloat32(i*size+4*3, true),
    },
    d: resources.getFloat32(i*size+4*4, true),
    k: resources.getUint32(i*size+4*5, true),
    a: resources.getUint32(i*size+4*6, true),
  }
}


const test_1 = () => {
  const universe = tml.Universe.new()
  const forest_k = universe.add_resource_kind(
    "tachicosmachines.forest",
    "Forest",
    "#0F0",
  )
  universe.add_machine_2(0.2, 0.3, 0.1, 0.1)
  universe.add_machine_2(0.8, 0.9, 0.1, 0.1)
  universe.add_resource(forest_k, 0.201, 0.301)
  universe.add_resource(forest_k, 0.8, 0.1)
  const machines = new DataView(
    tm.memory.buffer,
    universe.machines(),
    universe.machines_count() * 4 * machine_struct_size
  );
  assert_eq(machine(machines, 0).p.x, 0.2)
  assert_eq(machine(machines, 1).p.y, 0.9)

  const resources = new DataView(
    tm.memory.buffer,
    universe.resources(),
    universe.resources_count() * 4 * resource_struct_size
  );

  universe.tick();
  universe.tick();
  universe.tick();

  console.log( resource(resources, 0) )
  console.log( resource(resources, 1) )

  // assert_eq(resource(resources, 1).p.y, 0.9)

}


const test = () => {
  test_1()
}


const run = () => {
  const ZOOM = 2;
  const size = Math.min(document.documentElement.clientHeight, document.documentElement.clientWidth)
  const size_2 = Math.max(document.documentElement.clientHeight, document.documentElement.clientWidth)
  if (document.documentElement.clientHeight > document.documentElement.clientWidth) {
    document.body.style.flexDirection = 'column'
  } else {
    document.body.style.flexDirection = 'row'
  }
  // <!-- <label>acceleration: <span id="acceleration_value"></span></label> <input type="range" min="0" max="100" value="${((data.acceleration-data.acceleration_min) / ( data.acceleration_max - data.acceleration_min))*100}" class="slider" id="acceleration"> -->
  document.body.innerHTML = `
    <canvas id="canvas" width="${size*ZOOM}px" height="${size*ZOOM}px"></canvas>
    <div id="panel">
      <p>x: <span id="mouse_x"></span></p>
      <p>y: <span id="mouse_y"></span></p>
      <textarea id="logs"></textarea>
    </div>
  `
  const canvas = document.getElementById('canvas')
  const context = canvas.getContext("2d")
  const universe = tml.Universe.new()
  for (var i = 0; i < 100; i++) {
    universe.add_machine_2(Math.random(), Math.random(), 0.0, 0.0)
  }
  const forest_k = universe.add_resource_kind(
    "tachicosmachines.forest",
    "Forest",
    "#0F0",
  )
  for (var i = 0; i < 1000; i++) {
    universe.add_resource(forest_k, Math.random(), Math.random())
  }
  tick(universe)
  draw(universe, context)
}


const tick = (universe) => {
  universe.tick()
  if (keep_ticking) {
    setTimeout(() => {
      tick(universe)
    }, 0);
  }
}


const draw = (universe, context) => {
  clear(context)
  const machines = new DataView(
    tm.memory.buffer,
    universe.machines(),
    universe.machines_count() * 4 * machine_struct_size
  );
  for (let i = 0; i < universe.machines_count(); i++) {
    const m = machine(machines, i)
    fill_circle( context, m.p.x, m.p.y, m.d, "#F0F")
  }
  const resources = new DataView(
    tm.memory.buffer,
    universe.resources(),
    universe.resources_count() * 4 * resource_struct_size
  );
  for (let i = 0; i < universe.resources_count(); i++) {
    const r = resource(resources, i)
    if (r.a) {
      fill_circle( context, r.p.x, r.p.y, r.d, "#0F0")
    }
  }
  for (let i = 0; i < universe.machines_count(); i++) {
    const m = machine(machines, i)
    const r = resource(resources, m.t)
    if (r.a && m.ht) {
      line(context, m.p.x, m.p.y, r.p.x, r.p.y, 2, "#aaa")
    }
  }
  if (keep_drawing) {
    requestAnimationFrame(() => {
      draw(universe, context)
    })
  }
}


test()


run()
