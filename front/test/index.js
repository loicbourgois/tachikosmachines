import {
  fill_circle,
  line,
  clear,
  fill_rect,
} from "../canvas.js"
import {
  direction,
  translate,
  mul,
  rotate,
  distance_squared,
  distance,
} from "../math.js"


const canvas_resolution = 2.0
const keep_drawing = true
const keep_ticking = true
const data = {
  doots: [
    {
      op: {
        x: 0.499,
        y: 0.5,
      },
      p: {
        x: 0.5,
        y: 0.5,
      },
      d: 0.15,
      dests: new Array(6,{})
    }
  ]
}


const new_canvas = (canvas_resolution) => {
  const size = Math.min(document.documentElement.clientHeight, document.documentElement.clientWidth)
  const size_2 = Math.max(document.documentElement.clientHeight, document.documentElement.clientWidth)
  return `<canvas id="canvas" width="${size*canvas_resolution}px" height="${size*canvas_resolution}px"></canvas>`
}


const init = () => {
  document.body.style.flexDirection = document.documentElement.clientHeight > document.documentElement.clientWidth ? 'column' : 'row'
  document.body.innerHTML = `
    ${new_canvas(canvas_resolution)}
    <div id="panel">
      <p>x: <span id="mouse_x"></span></p>
      <p>y: <span id="mouse_y"></span></p>
    </div>
  `
}


const speed = 0.125
const tick = () => {
  data.doots[0].op.x = data.doots[0].p.x
  data.doots[0].op.y = data.doots[0].p.y
  data.doots[0].p.x = Math.sin(performance.now()*0.005 *speed) * 0.25 + 0.5
  data.doots[0].p.y = Math.cos(performance.now()*0.0095*speed) * 0.25 + 0.5
  data.doots[0].dir = direction(data.doots[0].op, data.doots[0].p)
  if (keep_ticking) {
    setTimeout(() => {
      tick()
    }, 10);
  }
}


const render = (context) => {
  clear(context)
  fill_rect(context, 0.5, 0.5, 1.0, 1.0, "#bb6")


  const p = data.doots[0].p
  const dir = data.doots[0].dir
  const d = data.doots[0].d
  const eye_left = rotate(translate(p, mul(dir, d*0.5)), p, -0.065)
  const eye_right = rotate(translate(p, mul(dir, d*0.5)), p, 0.065)
  const eye_left_ = rotate(translate(p, mul(dir, d*0.55)), p, -0.0625)
  const eye_right_ = rotate(translate(p, mul(dir, d*0.55)), p, 0.0625)


  const color_body = "#9dd"


  const oo = 0.3;
  const bases = [
    rotate(translate(p, mul(dir, d*0.5)), p, -0.15),
    rotate(translate(p, mul(dir, d*0.5)), p, -0.4),
    rotate(translate(p, mul(dir, d*0.5)), p, -0.275),
    rotate(translate(p, mul(dir, d*0.5)), p, 0.15),
    rotate(translate(p, mul(dir, d*0.5)), p, 0.4),
    rotate(translate(p, mul(dir, d*0.5)), p, 0.275),
  ]
  const ideals = bases.map(x=>translate(x, mul(direction(p,x), d*oo)))
  const ideals_front = ideals.map(x=>translate(x, mul(dir, d*oo*0.75)))
  const signs = [
    1,
    1,
    1,
    -1,
    -1,
    -1,
  ]


  const angled = (a,b, dist_, sign=1.0) => {
    const c = mul(translate(a,b),0.5)
    const b_ = Math.sqrt( dist_*0.5*dist_*0.5 -  distance_squared(a,b)*0.25 )
    return rotate( translate(c,   mul(direction(a,c),b_*sign)   ), c, 0.25)
  }


  for (var i = 0; i < bases.length; i++) {
    fill_circle(context, bases[i].x, bases[i].y, d*0.17, color_body )
    const full_leg_length = d*oo*2
    if (
        !data.doots[0].dests[i]?.x
    ){
      data.doots[0].dests[i] = ideals_front[i]
    }
    else if ( distance_squared(data.doots[0].dests[i], ideals[i] ) > full_leg_length*full_leg_length*0.2
        || distance_squared(data.doots[0].dests[i], p ) < d*d*0.35
    ) {
      data.doots[0].dests[i] =  translate( ideals[i], mul( direction(ideals[i], data.doots[0].dests[i] ), full_leg_length*-0.3 ) )
      //data.doots[0].dests[i] = ideals_front[i]
    }
    fill_circle(context, data.doots[0].dests[i].x, data.doots[0].dests[i].y, d*0.2, "#90d8")
    fill_circle(context, data.doots[0].dests[i].x, data.doots[0].dests[i].y, d*0.1, color_body)
    //const elbow = angled( bases[i], data.doots[0].dests[i], full_leg_length, signs[i] )

    const elbow = {
      x: (ideals[i].x*1.0 + data.doots[0].dests[i].x + bases[i].x ) / 3.0,
      y: (ideals[i].y*1.0 + data.doots[0].dests[i].y + bases[i].y ) / 3.0,
    }

    fill_circle(context, elbow.x, elbow.y, d*0.1, color_body)
    line(context, bases[i].x, bases[i].y, elbow.x, elbow.y, d*200, color_body)
    line(context, elbow.x, elbow.y, data.doots[0].dests[i].x, data.doots[0].dests[i].y, d*200, color_body)
    // fill_circle(context, ideals[i].x, ideals[i].y, full_leg_length, "#90d5")
  }


  fill_circle(context, p.x, p.y, d, color_body)
  fill_circle(context, eye_left.x, eye_left.y, d*0.4, "#fff")
  fill_circle(context, eye_right.x, eye_right.y, d*0.4, "#fff")
  fill_circle(context, eye_left_.x, eye_left_.y, d*0.2, "#222")
  fill_circle(context, eye_right_.x, eye_right_.y, d*0.2, "#222")



  if (keep_drawing) {
    requestAnimationFrame(() => {
      render(context)
    })
  }
}


const go = () => {
  init()
  tick()
  render(document.getElementById('canvas').getContext('2d'))
}


go()
