use sdl3::gpu::{
    Buffer, BufferBinding, BufferRegion, BufferUsageFlags, 
    ColorTargetDescriptionBuilder, ColorTargetInfo, CopyPass, 
    CullMode, Device, FillMode, GraphicsPipelineTargetInfo, IndexElementSize, 
    LoadOp, PrimitiveType, RasterizerState, ShaderFormat, StoreOp, 
    TransferBuffer, TransferBufferLocation, TransferBufferUsage, VertexAttribute, 
    VertexBufferDescription, VertexElementFormat, VertexInputRate, VertexInputState
};
use sdl3:: video::Window;

pub struct Renderer {
    window: Window,
    gpu: sdl3::gpu::Device,
    pipeline: sdl3::gpu::GraphicsPipeline,
    backround_color: sdl3::pixels::Color,
    vbo: Buffer,
    ibo: Buffer,
}

#[allow(unused)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct VertexPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

fn frect_to_verts(frect: sdl3::render::FRect, screen_width: f32, screen_height: f32) -> [VertexPosition; 4] {
    let x1 = (frect.x / screen_width) * 2.0 - 1.0;
    let y1 = 1.0 - (frect.y / screen_height) * 2.0;
    let x2 = ((frect.x + frect.w) / screen_width) * 2.0 - 1.0;
    let y2 = 1.0 - ((frect.y + frect.h) / screen_height) * 2.0;

    [
        VertexPosition { x: x1, y: y1, z: 0.0 }, // Top-left
        VertexPosition { x: x1, y: y2, z: 0.0 }, // Bottom-left
        VertexPosition { x: x2, y: y2, z: 0.0 }, // Bottom-right
        VertexPosition { x: x2, y: y1, z: 0.0 }, // Top-right
    ]
}

fn create_vbo_ibo(
    gpu: &Device,
    window: &Window,
    frects: &[sdl3::render::FRect],
    transfer_buffer: &TransferBuffer,
    copy_pass: &CopyPass,
) -> (Buffer, Buffer) {
    let (width, height) = window.size();

    let mut indicies: Vec<u16> = vec![];
    let mut n = 0;
    let verts = frects
    .iter()
    .map(|frect| {
        indicies.extend([0+n, 1+n, 2+n, 2+n, 3+n, 0+n]);
        n += 4;
        frect_to_verts(*frect, width as f32, height as f32)
    })
    .flat_map(|array| array.into_iter())
    .collect::<Vec<VertexPosition>>();
    (
        create_buffer_with_data(
            gpu,
            transfer_buffer,
            copy_pass,
            BufferUsageFlags::Vertex,
            &verts
        ),
        create_buffer_with_data(
            gpu,
            transfer_buffer,
            copy_pass,
            BufferUsageFlags::Index,
            &indicies
        )
    )
}


impl Renderer {
    pub fn new(window: sdl3::video::Window, frects: &[sdl3::render::FRect]) -> Renderer {
        let gpu = sdl3::gpu::Device::new(
            ShaderFormat::SpirV | ShaderFormat::Dxil | ShaderFormat::Dxbc | ShaderFormat::MetalLib,
            true,
        )
        .with_window(&window).expect("Device creation failed");

        let fs_source = include_bytes!("shaders/spv/quad.frag.spv");
        let vs_source = include_bytes!("shaders/spv/quad.vert.spv");

        let vs = gpu
            .create_shader()
            .with_code(
                sdl3::gpu::ShaderFormat::SpirV,
                vs_source,
                sdl3::gpu::ShaderStage::Vertex,
            )
            .with_uniform_buffers(1)
            .with_entrypoint("main")
            .build().expect("Vertex shader creation failed");

        let fs = gpu
            .create_shader()
            .with_code(
                sdl3::gpu::ShaderFormat::SpirV,
                fs_source,
                sdl3::gpu::ShaderStage::Fragment,
            )
            .with_entrypoint("main")
            .build().unwrap();

    
        let swapchain_format = gpu.get_swapchain_texture_format(&window);

        let pipeline = gpu
        .create_graphics_pipeline()
        .with_primitive_type(PrimitiveType::TriangleList)
        .with_fragment_shader(&fs)
        .with_vertex_shader(&vs)
        .with_vertex_input_state(
            VertexInputState::new()
                .with_vertex_buffer_descriptions(&[VertexBufferDescription::new()
                    .with_slot(0)
                    .with_pitch((size_of::<f32>() * 3) as u32) // 3 floats per vertex
                    .with_input_rate(VertexInputRate::Vertex)
                    .with_instance_step_rate(0)])
                .with_vertex_attributes(&[VertexAttribute::new()
                    .with_format(VertexElementFormat::Float3)
                    .with_location(0)
                    .with_buffer_slot(0)
                    .with_offset(0),
                ]),
        )
        .with_rasterizer_state(
            RasterizerState::new()
                .with_fill_mode(FillMode::Fill)
                .with_cull_mode(CullMode::None),
        )
        .with_target_info(
            GraphicsPipelineTargetInfo::new()
                .with_color_target_descriptions(&[ColorTargetDescriptionBuilder::new()
                    .with_format(swapchain_format)
                    .build()
                ])
        )
        .build().expect("Pipeline creation failed");
    
        vs.release(&gpu);
        fs.release(&gpu);
        // let frects = [
        //     sdl3::render::FRect::new(20.0, 20.0, 50.0, 50.0),
        //     sdl3::render::FRect::new(200.0, 200.0, 50.0, 50.0),
        //     sdl3::render::FRect::new(400.0, 200.0, 50.0, 50.0),
        //     sdl3::render::FRect::new(200.0, 400.0, 50.0, 50.0),
        // ];
        let vertices_len_bytes = frects.len() * 4 * size_of::<VertexPosition>();
        let indices_len_bytes = frects.len() * 4 * size_of::<u16>();
        let transfer_buffer = gpu
            .create_transfer_buffer()
            .with_size(vertices_len_bytes.max(indices_len_bytes) as u32)
            .with_usage(TransferBufferUsage::Upload)
            .build();
    
        let copy_commands = gpu.acquire_command_buffer();
        let copy_pass = gpu.begin_copy_pass(&copy_commands).expect("Failed to begin copy pass");

        let (vbo, ibo) = create_vbo_ibo(
            &gpu,
            &window,
            &frects,
            &transfer_buffer,
            &copy_pass,
        );

        transfer_buffer.release(&gpu);
        gpu.end_copy_pass(copy_pass);

        copy_commands.submit().expect("Failed to submit copy commands");

        let backround_color = sdl3::pixels::Color::RGBA(20, 20, 20, 255);

        Renderer {
            window,
            gpu,
            pipeline,
            backround_color,
            vbo,
            ibo,
        }
    }

    pub fn render_gpu(&mut self) -> Result<(), sdl3::Error> {
        let mut command_buffer = self.gpu.acquire_command_buffer();
        if let Ok(swapchain) = self.gpu.wait_and_acquire_swapchain_texture(&self.window, &mut command_buffer)
        {      
            let color_targets = [ColorTargetInfo::default()
            .with_texture(swapchain)
            .with_load_op(LoadOp::Clear)
            .with_store_op(StoreOp::Store)
            .with_clear_color(self.backround_color)];

            let render_pass = self.gpu.begin_render_pass(&command_buffer, &color_targets, None)?;

            render_pass.bind_graphics_pipeline(&self.pipeline);

            render_pass.bind_vertex_buffers(
            0,
            &[BufferBinding::new()
                .with_buffer(&self.vbo)
                .with_offset(0)],
            );
            render_pass.bind_index_buffer(
            &BufferBinding::new()
                .with_buffer(&self.ibo)
                .with_offset(0),
            IndexElementSize::_16Bit,
            );


            // Finally, draw the cube
            render_pass.draw_indexed_primitives(self.ibo.len() as u32, 1, 0, 0, 0);

            self.gpu.end_render_pass(render_pass);
            command_buffer.submit()?;
        } else {
            // Swapchain unavailable, cancel work
            command_buffer.cancel();
        }
        Ok(())
    }


    pub fn set_backround_color(&mut self, color: (u8, u8, u8)) {
        self.backround_color = sdl3::pixels::Color::RGB(color.0, color.1, color.2)
    }
}

fn create_buffer_with_data<T: Copy>(
    gpu: &Device,
    transfer_buffer: &TransferBuffer,
    copy_pass: &CopyPass,
    usage: BufferUsageFlags,
    data: &[T],
) -> Buffer {
    let len_bytes = data.len() * std::mem::size_of::<T>();

    let buffer = gpu
        .create_buffer()
        .with_size(len_bytes as u32)
        .with_usage(usage)
        .build();

    let mut map = transfer_buffer.map::<T>(gpu, true);
    let mem = map.mem_mut();
    for (index, &value) in data.iter().enumerate() {
        mem[index] = value;
    }

    map.unmap();

    copy_pass.upload_to_gpu_buffer(
        TransferBufferLocation::new()
            .with_offset(0)
            .with_transfer_buffer(transfer_buffer),
        BufferRegion::new()
            .with_offset(0)
            .with_size(len_bytes as u32)
            .with_buffer(&buffer),
        true,
    );

    buffer
}