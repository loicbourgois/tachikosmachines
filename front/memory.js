import {
  machine_struct_size,
  resource_struct_size,
  keep_drawing,
  keep_ticking,
  MACHINES,
} from "./constants.js"


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


export {
  machine,
  resource,
}
