import {
  fill_circle,
  fill_rect,
  line,
  fill_text,
  clear,
} from "./canvas.js"
import {
  machine,
  resource,
} from "./memory.js"
import {
  machine_struct_size,
  resource_struct_size,
  keep_drawing,
  keep_ticking,
  MACHINES,
} from "./constants.js"


const draw = (
  universe,
  context,
  colors,
  tm,
  data,
  context_charts,
) => {
  // clear(context)
  clear(context_charts)
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
    if ( r.a ) {
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
  const machines_count = universe.machines_count()
  for (let i = 0; i < machines_count; i++) {
    const m = machine(machines, machines_stores, kind_count, i)
    if (m.active) {
      fill_circle( context, m.p.x, m.p.y, m.d, "#F0F")
      let y = 0.0
      for (var j = 0; j < kind_count; j++) {
        const v = m.store[j]
        const y2 = y + v / kind_count
        const x = (i+0.5)/machines_count
        line(context_charts, x, y, x, y2, Math.max(1, context_charts.canvas.width / (machines_count)) , colors[j]({store:1.0}) )
        y = y2
      }
    }
  }
  if (data.mouse.x && data.mouse.y) {
    document.getElementById("mouse_x").innerHTML = data.mouse.x.toFixed(2)
    document.getElementById("mouse_y").innerHTML = data.mouse.y.toFixed(2)
  }
  if (keep_drawing) {
    requestAnimationFrame(() => {
      requestAnimationFrame(() => {
        draw(universe, context, colors, tm, data, context_charts)
      })
    })
  }
}


export {
  draw,
}
