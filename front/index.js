import * as tml from "../wasm/tachikosmachines.js";
const tm = await tml.default()


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
import {
  test
} from "./test.js"
import {
  machine_struct_size,
  resource_struct_size,
  keep_drawing,
  keep_ticking,
} from "./constants.js"
import {
  machine,
  resource,
} from "./memory.js"
import {
  draw,
} from "./draw.js"


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
      <canvas id="canvas_charts" width="${200}px" height="${100}px"></canvas>
      <!-- <textarea id="logs"></textarea> -->
    </div>
  `
  const canvas = document.getElementById('canvas')
  handle_mouse_event(canvas, data.mouse, canvas_mouse_click)
  const context = canvas.getContext("2d")
  const context_charts = document.getElementById('canvas_charts').getContext("2d")
  const universe = tml.Universe.new(
    0.02,
    0.0001,
    0.2,
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
  for (var i = 0; i < 1; i++) {
    universe.add_machine_2(Math.random(), Math.random(), 0.0, 0.0)
  }
  for (var i = 0; i < 100; i++) {
    universe.add_resource(resource_kinds.forest.u, Math.random(), Math.random())
    universe.add_resource(resource_kinds.bush.u, Math.random(), Math.random())
    universe.add_resource(resource_kinds.oa.u, Math.random(), Math.random())
  }
  tick(universe)
  draw(universe, context, colors, tm, data, context_charts)
}


const tick = (universe) => {
  universe.tick()
  if (keep_ticking) {
    setTimeout(() => {
      tick(universe)
    }, 0);
  }
}


test(tm, tml)


run()
