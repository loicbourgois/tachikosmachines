const ZOOM = 0.75;


const fill_text = (context, text, x, y) => {
  const size = 18
  const xx = context.canvas.width * x * ZOOM - text.length*16/3  ;
  const yy = context.canvas.height - context.canvas.height * y + size*0.45;
  context.font = `${size}px monospace`;
  context.fillStyle = "#fff"
  context.fillText(text, xx, yy);
}


const line = (context, x1, y1, x2, y2, lineWidth, color) => {
  const xx1 = context.canvas.width * x1;
  const yy1 = context.canvas.height - context.canvas.height * y1;
  const xx2 = context.canvas.width * x2;
  const yy2 = context.canvas.height - context.canvas.height * y2;
  context.lineWidth = lineWidth?lineWidth:5;
  context.strokeStyle = color ? color : "#fff4";
  context.beginPath()
  context.moveTo(xx1, yy1)
  context.lineTo(xx2, yy2)
  context.stroke()
}


const fill_rect = (context, x, y, width, height, color) => {
  const ww = width * context.canvas.width;
  const hh = height * context.canvas.height;
  const xx = context.canvas.width * x - ww * 0.5;
  const yy = context.canvas.height - context.canvas.height * y - hh * 0.5;
  context.fillStyle = color;
  context.fillRect(xx, yy, ww, hh);
  context.lineWidth = 0;
}


const fill_circle = (context, x, y, diameter, color) => {
  const xx = context.canvas.width * x;
  const yy = context.canvas.height - context.canvas.height * y;
  const radius = diameter * context.canvas.width * 0.5;
  context.beginPath();
  context.arc(xx, yy, radius, 0, 2 * Math.PI, false);
  context.fillStyle = color;
  context.fill();
}


const clear = (context) => {
  context.clearRect(0,0,context.canvas.width, context.canvas.height)
}

export {
  fill_circle,
  fill_rect,
  line,
  fill_text,
  clear,
}
