import * as tml from "../wasm/tachikosmachines.js";
import {
  fill_circle,
  line,
  clear,
  fill_rect,
}
from "./canvas.js"
import {
  handle_mouse_event
} from "./mouse.js"


const tm = await tml.default()
const machine_struct_size = 16 //13 + 1
const resource_struct_size = 8
const keep_drawing = true
const keep_ticking = true


const assert_eq = (a, b, message) => {
    if ( Math.abs(a - b) > 0.001 ) {
        throw (message || "Assertion failed" ) + `Got ${a}, Expected ${b}`;
    }
}


const machine = (
  machines,
  machines_stores,
  kind_count,
  i
) => {
  const size = machine_struct_size*4
  let store = []
  for (var j = 0; j < kind_count; j++) {
    const aa = (j + i*kind_count) * 4
    store.push( machines_stores.getFloat32( aa,true  ) )
  }
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
    store: store,
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
    store: resources.getFloat32(i*size+4*7, true),
  }
}


const test_1 = () => {
  const universe = tml.Universe.new(0.01)
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


const test = () => {
  test_1()
}



const canvas_mouse_click = (mouse) => {
  console.log(`click ${mouse.x}`)
}


const data = {
  mouse: {}
}
const run = () => {
  const ZOOM = 0.25;
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

  handle_mouse_event(canvas, data.mouse, canvas_mouse_click)

  const context = canvas.getContext("2d")
  const universe = tml.Universe.new(
    0.01,
  )
  const colors = {}
  const resource_kinds = {
    forest: {
      text_id: "tachicosmachines.forest",
      label: "Forest",
      color_str: "-",
      color_f: (x) => {
        const cr = 255.0 * (1.0 - x.store)
        const cg = 155.0
        const cb = 0.0
        return `rgba(${cr}, ${cg}, ${cb}, 1.0)`
      },
      growth_rate: 0.005,
    },
    bush: {
      text_id: "tachicosmachines.bush",
      label: "Bush",
      color_str: "-",
      color_f: (x) => {
        const cr = 255.0 * (1.0 - x.store)
        const cg = 255.0
        const cb = 255.0
        return `rgba(${cr}, ${cg}, ${cb}, 1.0)`
      },
      growth_rate: 0.005,
    },
    oa: {
      text_id: "tachicosmachines.oa",
      label: "Oa",
      color_str: "-",
      color_f: (x) => {
        const cr = 255.0
        const cg = 255.0 * (1.0 - x.store*0.5)
        const cb = 255.0 * (1.0 - x.store)
        return `rgba(${cr}, ${cg}, ${cb}, 1.0)`
      },
      growth_rate: 0.005,
    }
  }
  for (let k of Object.keys(resource_kinds)) {
    const resource_kind = resource_kinds[k]
    const u = universe.add_resource_kind(
      resource_kind.text_id,
      resource_kind.label,
      resource_kind.color_str,
      resource_kind.growth_rate,
    )
    colors[u] = resource_kind.color_f
    resource_kinds[k].u = u
  }
  for (var i = 0; i < 300; i++) {
    universe.add_machine_2(Math.random(), Math.random(), 0.0, 0.0)
  }
  for (var i = 0; i < 100; i++) {
    universe.add_resource(resource_kinds.forest.u, Math.random(), Math.random())
    universe.add_resource(resource_kinds.bush.u, Math.random(), Math.random())
    universe.add_resource(resource_kinds.oa.u, Math.random(), Math.random())
  }
  tick(universe)
  draw(universe, context, colors)
}


const tick = (universe) => {
  universe.tick()
  if (keep_ticking) {
    setTimeout(() => {
      tick(universe)
    }, 0);
  }
}


const draw = (universe, context, colors) => {
  // clear(context)
  fill_rect(context, 0.5, 0.5, 1.0, 1.0, "#0002")
  const machines = new DataView(
    tm.memory.buffer,
    universe.machines(),
    universe.machines_count() * 4 * machine_struct_size
  );
  const resources = new DataView(
    tm.memory.buffer,
    universe.resources(),
    universe.resources_count() * 4 * resource_struct_size
  );
  const machines_stores = new DataView(
    tm.memory.buffer,
    universe.machines_stores(),
    universe.machines_stores_count() * 4
  );
  const kind_count = universe.resources_kind_count();

  for (let i = 0; i < universe.resources_count(); i++) {
    const r = resource(resources, i)
    if (r.a) {
      fill_circle( context, r.p.x, r.p.y, r.d, colors[r.k](r))
    }
  }
  for (let i = 0; i < universe.machines_count(); i++) {
    const m = machine(machines, machines_stores, kind_count, i)
    const r = resource(resources, m.t)
    if (r.a && m.ht) {
      // line(context, m.p.x, m.p.y, r.p.x, r.p.y, 1, "#aaa")
    }
  }
  for (let i = 0; i < universe.machines_count(); i++) {
    const m = machine(machines, machines_stores, kind_count, i)
    fill_circle( context, m.p.x, m.p.y, m.d, "#F0F")
    // if (i==0) {
    //   console.log(m.store)
    // }
  }

  // console.log(data.mouse)

  if (data.mouse.x) {
    document.getElementById("mouse_x").innerHTML = data.mouse.x.toFixed(2)
    document.getElementById("mouse_y").innerHTML = data.mouse.y.toFixed(2)
  }


  if (keep_drawing) {
    requestAnimationFrame(() => {
      requestAnimationFrame(() => {
        draw(universe, context, colors)
      })
    })
  }
}


test()


run()
