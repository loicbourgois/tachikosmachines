const handle_mouse_event = (canvas, mouse, on_click_handler) => {
  canvas.addEventListener('mousemove', (evt) => {
    const rect = canvas.getBoundingClientRect();
    mouse['x'] = (evt.clientX - rect.left) / rect.width
    mouse.y = 1.0 - (evt.clientY - rect.top) / rect.height
  }, false);
  canvas.addEventListener('click', (evt) => {
    const rect = canvas.getBoundingClientRect();
    mouse.x = (evt.x - rect.left) / rect.width
    mouse.y = 1.0 - (evt.y - rect.top) / rect.height
    on_click_handler(mouse)
  })
}

export {
  handle_mouse_event,
}
