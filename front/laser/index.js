import {
  new_canvas,
} from '../canvas.js'
import {
  gpu_init,
} from './gpu.js'


const canvas_resolution = 1.0;


const init = () => {
  document.body.style.flexDirection = document.documentElement.clientHeight > document.documentElement.clientWidth ? 'column' : 'row'
  document.body.innerHTML = `
    ${new_canvas(canvas_resolution, 'webgpu')}
    <div id="panel">
      <p>x: <span id="mouse_x"></span></p>
      <p>y: <span id="mouse_y"></span></p>
    </div>
  `
}


const go = () => {
  init()
  gpu_init(3, 'webgpu')

  // gpu_render({
  //   buffer: gpu.buffers.in,
  //   device: gpu.device,
  //   adapter: gpu.adapter
  // })

  // tick()
  // render(document.getElementById('canvas').getContext('2d'))
}


go()
