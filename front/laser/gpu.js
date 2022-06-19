const gpu_init = async (
  buffer_size,
  canvas_id,
) => {
  if (!navigator.gpu) throw Error("WebGPU not supported.");

  if (! ("gpu" in navigator) ) {
    const m = "Application requires WebGPU.\nInstructions on how to enable at https://web.dev/gpu/#use"
    alert(m)
    console.error(m)
    return
  }
  const r = {}
  const canvas = document.getElementById(canvas_id)
  const context = canvas.getContext('webgpu', {antialias: true});
  const presentation_size = [
    canvas.clientWidth,
    canvas.clientHeight,
  ];
  const presentation_format = context.getPreferredFormat(adapter);
  context.configure({
    device,
    format: presentation_format,
    size: presentation_size,
  });



  r.adapter = await navigator.gpu.requestAdapter();
  if (!r.adapter) {
    console.error("No gpu adapter found")
    return;
  }
  r.device = await r.adapter.requestDevice();
  r.buffers = {
    write_in: r.device.createBuffer({
      size: buffer_size,
      usage: GPUBufferUsage.MAP_WRITE | GPUBufferUsage.COPY_SRC
    }),
    // reset: gpu.device.createBuffer({
    //   size: buffer_size({cell_count:cell_count}),
    //   usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST | GPUBufferUsage.COPY_SRC
    // }),
    in: r.device.createBuffer({
      size: buffer_size,
      usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST | GPUBufferUsage.COPY_SRC
    }),
    // out: gpu.device.createBuffer({
    //   size: buffer_size({cell_count:cell_count}),
    //   usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST | GPUBufferUsage.COPY_SRC
    // }),
    // read: gpu.device.createBuffer({
    //   size: buffer_size({cell_count:cell_count}),
    //   usage: GPUBufferUsage.MAP_READ | GPUBufferUsage.COPY_DST
    // })
  }
  r.bind_group_layouts = {
    render: device.createBindGroupLayout({
      entries: [
        { // In
          binding: 0,
          visibility: GPUShaderStage.FRAGMENT,
          buffer: {
            type: "storage"
          }
        },
      ]
    })
  }
  r.bind_group = {
    render: device.createBindGroup({
      layout: r.bind_group_layouts.render,
      entries: [
        {
          binding: 0,
          resource: {
            buffer: r.buffers.in
          }
        },
      ]
    })
  }
  r.pipelines = {
    render: r.device.createRenderPipeline({
      vertex: {
        module: r.device.createShaderModule({
          code: vertex_shader,
        }),
        entryPoint: 'main',
      },
      fragment: {
        module: device.createShaderModule({
          code: fragment_shader({
            canvas: canvas,
            cell_count: cell_count
          }),
        }),
        entryPoint: 'main',
        targets: [
          {
            format: context.getPreferredFormat(r.adapter),
          },
        ],
      },
      primitive: {
        topology: 'triangle-list',
      },
      layout: r.device.createPipelineLayout({
        bindGroupLayouts: [r.bind_group_layouts.render]
      }),
    })
  }



  return r
}


export {
  gpu_init,
}
