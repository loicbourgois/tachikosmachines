import * as tml from "../wasm/tachikosmachines.js";
const tm = await tml.default()


const assert_eq = (a, b, message) => {
    if ( Math.abs(a - b) > 0.0000001 ) {
        throw (message || "Assertion failed" ) + `Got ${a}, Expected ${b}`;
    }
}


const machine = (machines, i) => {
  const size = 10*4
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
  }
}


const test = () => {
  const universe = tml.Universe.new()
  universe.test()
  universe.add_machine_2(0.2, 0.3, 0.1, 0.1)
  universe.add_machine_2(0.8, 0.9, 0.1, 0.1)
  const machines = new DataView(tm.memory.buffer, universe.machines(), 2*4*10);
  console.log(machine(machines, 0))
  console.log(machine(machines, 1))
  assert_eq(machine(machines, 0).p.x, 0.2)
  assert_eq(machine(machines, 1).p.y, 0.9)
}


test()
