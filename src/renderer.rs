pub struct Renderer {
    gpu: sdl3::gpu::Device,
    pipeline: sdl3::gpu::GraphicsPipeline,
    backround_color: sdl3::pixels::Color,
}

// Renderer should be created 
impl Renderer {
    pub fn new(window: &sdl3::video::Window) -> Renderer {
        let gpu = sdl3::gpu::Device::new(
            sdl3::gpu::ShaderFormat::SpirV,
            true
        ).with_window(window).expect("GPU-device creation failed");

        let fs_source = include_bytes!("shaders/triangle.frag.spv");
        let vs_source = include_bytes!("shaders/triangle.vert.spv");

        let vs = gpu
        .create_shader()
        .with_code(
            sdl3::gpu::ShaderFormat::SpirV,
            vs_source,
            sdl3::gpu::ShaderStage::Vertex,
        )
        .with_entrypoint("main")
        .build().unwrap();

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

        // Create a pipeline, we specify that we want our target format in the one of the swapchain
        // since we are rendering directly unto the swapchain, however, we could specify one that
        // is different from the swapchain (i.e offscreen rendering)
        let color_target = [sdl3::gpu::ColorTargetDescriptionBuilder::new()
            .with_format(swapchain_format)
            .build()];

        let pipeline = gpu
            .create_graphics_pipeline()
            .with_fragment_shader(&fs)
            .with_vertex_shader(&vs)
            .with_primitive_type(sdl3::gpu::PrimitiveType::TriangleList)
            .with_fill_mode(sdl3::gpu::FillMode::Fill)
            .with_target_info(&color_target)
            .build()
            .expect("Render pipeline creation failed");
        drop(color_target);

        vs.release(&gpu);
        fs.release(&gpu);

        let backround_color = sdl3::pixels::Color::RGBA(20, 20, 20, 255);
        Renderer {
            gpu,
            pipeline,
            backround_color
        }
    }
    pub fn render(&self, window: &sdl3::video::Window) -> Result<(), sdl3::Error> {
        let mut command_buffer = self.gpu.acquire_command_buffer();
        if let Ok(swapchain) = self.gpu.wait_and_acquire_swapchain_texture(window, &mut command_buffer)
        {
            let color_targets = [
                sdl3::gpu::ColorTargetInfo::default()
                    .with_texture(swapchain) // Use swapchain texture
                    .with_load_op(sdl3::gpu::LoadOp::Clear) // Clear when load
                    .with_store_op(sdl3::gpu::StoreOp::Store) // Store back
                    .with_clear_color(self.backround_color), //blue with small RG bias
            ];
            // Here we do all (none) of our drawing (clearing the screen)
            let render_pass = self.gpu.begin_render_pass(&command_buffer, &color_targets, None)?;

            render_pass.bind_graphics_pipeline(&self.pipeline);
            // Screen is cleared here due to the color target info
            // Now we'll draw the triangle primitives
            render_pass.draw_primitives(3, 1, 0, 0); 

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