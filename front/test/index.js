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
      d: 0.05,
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


const speed = 0.25
const tick = () => {
  data.doots[0].op.x = data.doots[0].p.x
  data.doots[0].op.y = data.doots[0].p.y
  data.doots[0].p.x = Math.sin(performance.now()*0.002 *speed) * 0.25 + 0.5
  data.doots[0].p.y = Math.cos(performance.now()*0.00095*speed) * 0.25 + 0.5
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


  const oo = 0.5;
  const bases = [
    rotate(translate(p, mul(dir, d*0.5)), p, -0.165),
    rotate(translate(p, mul(dir, d*0.5)), p, -0.365),
    rotate(translate(p, mul(dir, d*0.5)), p, -0.215),
    rotate(translate(p, mul(dir, d*0.5)), p, 0.165),
    rotate(translate(p, mul(dir, d*0.5)), p, 0.365),
    rotate(translate(p, mul(dir, d*0.5)), p, 0.215),
  ]

  //const dir_2 = rotate(dir, {x:0,y:0}, 0.125)
  // const tr = translate(p, mul(dir, oo ))

  const ideals = [
    translate(bases[0], mul(direction(p,bases[0] ), d*oo)),
    translate(bases[1], mul(direction(p,bases[1] ), d*oo)),
    translate(bases[2], mul(direction(p,bases[2] ), d*oo)),
    translate(bases[3], mul(direction(p,bases[3] ), d*oo)),
    translate(bases[4], mul(direction(p,bases[4] ), d*oo)),
    translate(bases[5], mul(direction(p,bases[5] ), d*oo)),
  ]

  const ideals_front = [
    translate(ideals[0], mul(dir, d*oo*0.75)),
    translate(ideals[1], mul(dir, d*oo*0.75)),
    translate(ideals[2], mul(dir, d*oo*0.25)),
    translate(ideals[3], mul(dir, d*oo*0.75)),
    translate(ideals[4], mul(dir, d*oo*0.75)),
    translate(ideals[5], mul(dir, d*oo*0.25)),
  ]


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
    fill_circle(context, bases[i].x, bases[i].y, d*0.15, color_body )
    //  fill_circle(context, ideals[i].x, ideals[i].y, d*0.2, "#90d8")

    // line(context, bases[i].x, bases[i].y, ideals[i].x, ideals[i].y, 10, color_body)


    const full_leg_length = d*oo*2

    if (
        !data.doots[0].dests[i]?.x
        || distance_squared(data.doots[0].dests[i], ideals[i] ) > full_leg_length*full_leg_length*0.25
        || distance_squared(data.doots[0].dests[i], p ) < d*d*0.25
    ) {
      data.doots[0].dests[i] = ideals_front[i]
    }

    fill_circle(context, data.doots[0].dests[i].x, data.doots[0].dests[i].y, d*0.2, "#90d8")
    fill_circle(context, data.doots[0].dests[i].x, data.doots[0].dests[i].y, d*0.1, color_body)
    //line(context, bases[i].x, bases[i].y, data.doots[0].dests[i].x, data.doots[0].dests[i].y, 10, color_body)

    const elbow = angled( bases[i], data.doots[0].dests[i], full_leg_length, signs[i] )
    fill_circle(context, elbow.x, elbow.y, d*0.1, color_body)

    line(context, bases[i].x, bases[i].y, elbow.x, elbow.y, 20, color_body)
    line(context, elbow.x, elbow.y, data.doots[0].dests[i].x, data.doots[0].dests[i].y, 14, color_body)
//fill_circle(context, ideals[i].x, ideals[i].y, d*0.1, color_body)

    //fill_circle(context, ideals[i].x, ideals[i].y, full_leg_length, "#90d5")
    //
    // fill_circle(context, ideals_front[i].x, ideals_front[i].y, d*0.2, color_body)

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
