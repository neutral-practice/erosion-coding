// Copyright (c) 2016 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.
#![allow(warnings)] // not today, erosion

mod display_mods;
use display_mods::{
    display_time_elapsed_nice, record_nanos, wait_one_millis_and_micros_and_nanos, Groupable,
};

mod f32_3;

mod u_modular;

mod magma_ocean;
use magma_ocean::{magma, petrify, Normal, Position, Stone};

mod moving_around;
use moving_around::move_forwards;

use cgmath::{Matrix3, Matrix4, Point3, Rad, Vector3};

// mod x;
// use x::{Normal, Position, INDICES, NORMALS, POSITIONS};

use std::{sync::Arc, time::Instant};
use vulkano::{
    buffer::{
        allocator::{SubbufferAllocator, SubbufferAllocatorCreateInfo},
        Buffer, BufferCreateInfo, BufferUsage, Subbuffer,
    },
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage,
        RenderPassBeginInfo,
    },
    descriptor_set::{
        allocator::StandardDescriptorSetAllocator,
        allocator::StandardDescriptorSetAllocatorCreateInfo, PersistentDescriptorSet,
        WriteDescriptorSet,
    },
    device::{
        physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, DeviceOwned,
        QueueCreateInfo, QueueFlags,
    },
    format::Format,
    image::{view::ImageView, Image, ImageCreateInfo, ImageType, ImageUsage},
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
    pipeline::{
        graphics::{
            color_blend::{ColorBlendAttachmentState, ColorBlendState},
            depth_stencil::{DepthState, DepthStencilState},
            input_assembly::InputAssemblyState,
            multisample::MultisampleState,
            rasterization::RasterizationState,
            vertex_input::{Vertex, VertexDefinition},
            viewport::{Viewport, ViewportState},
            GraphicsPipelineCreateInfo,
        },
        layout::PipelineDescriptorSetLayoutCreateInfo,
        GraphicsPipeline, Pipeline, PipelineBindPoint, PipelineLayout,
        PipelineShaderStageCreateInfo,
    },
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass},
    shader::EntryPoint,
    swapchain::{
        acquire_next_image, Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo,
    },
    sync::{self, GpuFuture},
    Validated, VulkanError, VulkanLibrary,
};
use winit::{
    dpi::{LogicalPosition, LogicalSize},
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    keyboard::{Key, ModifiersState},
    // WARNING: This is not available on all platforms (for example on the web).
    platform::modifier_supplement::KeyEventExtModifierSupplement,
    raw_window_handle::HasRawWindowHandle,
    window::{Fullscreen, Window, WindowBuilder, WindowId},
};

fn main() {
    let mut duration_since_epoch_nanos = record_nanos();
    // Statements here are executed when the compiled binary is called.

    // let warning_test = "unused"; // results in CI warning

    println!(
        "Welcome to the runtime started at {}",
        duration_since_epoch_nanos.group_with_nothing()
    );

    let stone = petrify(magma(2, 10.0));
    let pebble = petrify(magma(2, 2.0));

    ///|||\\\///|||\\\///|||\\\///|||\\\///|||\\\///|||\\\[ Main ]///|||\\\///|||\\\///|||\\\///|||\\\///|||\\\///|||\\\
    ///|||\\\
    ///|||\\\
    ///|||\\\
    let event_loop = EventLoop::new().unwrap();

    let library = VulkanLibrary::new().unwrap();
    let required_extensions = Surface::required_extensions(&event_loop);
    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            enabled_extensions: required_extensions,
            ..Default::default()
        },
    )
    .unwrap();

    let window = Arc::new(
        WindowBuilder::new()
            .with_fullscreen(Some(Fullscreen::Borderless(None)))
            .build(&event_loop)
            .unwrap(),
    );

    let surface = Surface::from_window(instance.clone(), window.clone()).unwrap();

    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };
    let (physical_device, queue_family_index) = instance
        .enumerate_physical_devices()
        .unwrap()
        .filter(|p| p.supported_extensions().contains(&device_extensions))
        .filter_map(|p| {
            p.queue_family_properties()
                .iter()
                .enumerate()
                .position(|(i, q)| {
                    q.queue_flags.intersects(QueueFlags::GRAPHICS)
                        && p.surface_support(i as u32, &surface).unwrap_or(false)
                })
                .map(|i| (p, i as u32))
        })
        .min_by_key(|(p, _)| match p.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 0,
            PhysicalDeviceType::IntegratedGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            PhysicalDeviceType::Other => 4,
            _ => 5,
        })
        .unwrap();

    println!(
        "Using device: {} (type: {:?})",
        physical_device.properties().device_name,
        physical_device.properties().device_type,
    );

    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            enabled_extensions: device_extensions,
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
    .unwrap();

    let queue = queues.next().unwrap();

    let (mut swapchain, images) = {
        let surface_capabilities = device
            .physical_device()
            .surface_capabilities(&surface, Default::default())
            .unwrap();
        let image_format = device
            .physical_device()
            .surface_formats(&surface, Default::default())
            .unwrap()[0]
            .0;

        Swapchain::new(
            device.clone(),
            surface,
            SwapchainCreateInfo {
                min_image_count: surface_capabilities.min_image_count.max(2),
                image_format,
                image_extent: window.inner_size().into(),
                image_usage: ImageUsage::COLOR_ATTACHMENT,
                composite_alpha: surface_capabilities
                    .supported_composite_alpha
                    .into_iter()
                    .next()
                    .unwrap(),
                ..Default::default()
            },
        )
        .unwrap()
    };

    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

    let (vertex_buffer, normals_buffer, index_buffer) =
        load_buffers_short(stone, memory_allocator.clone());

    let (vertex_buffer2, normals_buffer2, index_buffer2) =
        load_buffers_short(pebble, memory_allocator.clone());

    let uniform_buffer = SubbufferAllocator::new(
        memory_allocator.clone(),
        SubbufferAllocatorCreateInfo {
            buffer_usage: BufferUsage::UNIFORM_BUFFER,
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
    );

    let render_pass = vulkano::single_pass_renderpass!(
        device.clone(),
        attachments: {
            color: {
                format: swapchain.image_format(),
                samples: 1,
                load_op: Clear,
                store_op: Store,
            },
            depth_stencil: {
                format: Format::D16_UNORM,
                samples: 1,
                load_op: Clear,
                store_op: DontCare,
            },
        },
        pass: {
            color: [color],
            depth_stencil: {depth_stencil},
        },
    )
    .unwrap();

    let vs = vs::load(device.clone())
        .unwrap()
        .entry_point("main")
        .unwrap();
    let fs = fs::load(device.clone())
        .unwrap()
        .entry_point("main")
        .unwrap();

    let (mut pipeline, mut framebuffers) = window_size_dependent_setup(
        memory_allocator.clone(),
        vs.clone(),
        fs.clone(),
        &images,
        render_pass.clone(),
    );
    let mut recreate_swapchain = false;

    let mut previous_frame_end = Some(sync::now(device.clone()).boxed());
    let rotation_start = Instant::now();

    let descriptor_set_allocator = StandardDescriptorSetAllocator::new(
        device.clone(),
        StandardDescriptorSetAllocatorCreateInfo::default(),
    );
    let command_buffer_allocator =
        StandardCommandBufferAllocator::new(device.clone(), Default::default());

    //\\\|||///
    //\\\|||///
    //\\\|||///
    //\\\|||///\\\|||///\\\|||///\\\|||///\\\|||///[ the end of setup ]\\\|||///\\\|||///\\\|||///\\\|||///\\\|||///\\\|||///

    let mut rot_static = false;

    let mut view_point = Position {
        position: [0.0, -1.0, 1.0],
    };

    let mut center = Position {
        position: [0.0, 0.0, 0.0],
    };

    let mut up_direction = Position {
        position: [0.0, 0.0, 1.0],
    };

    duration_since_epoch_nanos = display_time_elapsed_nice(duration_since_epoch_nanos);

    //|||\\\///|||\\\///|||\\\///|||\\\///|||\\\///|||\\\[ loop ]///|||\\\///|||\\\///|||\\\///|||\\\///|||\\\///|||\\\///|||\\\
    //|||\\\
    //|||\\\
    //|||\\\

    event_loop.run(move |event, control_flow| {
        // duration_since_epoch_nanos = display_time_elapsed_nice(duration_since_epoch_nanos);
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::CloseRequested => {
                    control_flow.exit();
                }
                WindowEvent::Resized(_) => {
                    recreate_swapchain = true;
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    if event.state == ElementState::Pressed && !event.repeat {
                        match event.key_without_modifiers().as_ref() {
                            Key::Character("w") => {
                                // if modifiers.shift_key() {
                                //     println!("Shift + 1 | logical_key: {:?}", event.logical_key);
                                // } else {
                                println!("w");
                                move_forwards(&mut view_point, &mut center, &mut up_direction, 0.1);
                                // }
                            }
                            Key::Character("a") => {
                                // if modifiers.shift_key() {
                                //     println!("Shift + 1 | logical_key: {:?}", event.logical_key);
                                // } else {
                                println!("a");

                                // }
                            }
                            Key::Character("s") => {
                                // if modifiers.shift_key() {
                                //     println!("Shift + 1 | logical_key: {:?}", event.logical_key);
                                // } else {
                                println!("s");
                                move_forwards(
                                    &mut view_point,
                                    &mut center,
                                    &mut up_direction,
                                    -0.1,
                                );
                                // }
                            }
                            Key::Character("d") => {
                                // if modifiers.shift_key() {
                                //     println!("Shift + 1 | logical_key: {:?}", event.logical_key);
                                // } else {
                                println!("d");

                                // }
                            }
                            _ => (),
                        }
                    }
                }

                WindowEvent::RedrawRequested => {
                    let image_extent: [u32; 2] = window.inner_size().into();

                    if image_extent.contains(&0) {
                        return;
                    }

                    previous_frame_end.as_mut().unwrap().cleanup_finished();

                    if recreate_swapchain {
                        let (new_swapchain, new_images) = swapchain
                            .recreate(SwapchainCreateInfo {
                                image_extent,
                                ..swapchain.create_info()
                            })
                            .expect("failed to recreate swapchain");

                        swapchain = new_swapchain;
                        let (new_pipeline, new_framebuffers) = window_size_dependent_setup(
                            memory_allocator.clone(),
                            vs.clone(),
                            fs.clone(),
                            &new_images,
                            render_pass.clone(),
                        );
                        pipeline = new_pipeline;
                        framebuffers = new_framebuffers;
                        recreate_swapchain = false;
                    }

                    let uniform_buffer_subbuffer = {
                        let elapsed = rotation_start.elapsed();
                        let mut rotation = 0.0;
                        if !rot_static {
                            rotation = elapsed.as_secs() as f64
                                + elapsed.subsec_nanos() as f64 / 1_000_000_000.0;
                        }
                        let rotation = Matrix3::from_angle_y(Rad(rotation as f32));

                        // note: this teapot was meant for OpenGL where the origin is at the lower left
                        //       instead the origin is at the upper left in Vulkan, so we reverse the Y axis
                        let aspect_ratio =
                            swapchain.image_extent()[0] as f32 / swapchain.image_extent()[1] as f32;
                        let proj = cgmath::perspective(
                            Rad(std::f32::consts::FRAC_PI_2),
                            aspect_ratio,
                            0.01,
                            100.0,
                        );

                        let mut view = Matrix4::look_at_rh(
                            Point3::new(
                                view_point.position[0],
                                view_point.position[1],
                                view_point.position[2],
                            ),
                            Point3::new(center.position[0], center.position[1], center.position[2]),
                            Vector3::new(
                                up_direction.position[0],
                                up_direction.position[1],
                                up_direction.position[2],
                            ),
                        );

                        let scale = Matrix4::from_scale(0.01);

                        let uniform_data = vs::Data {
                            world: Matrix4::from(rotation).into(),
                            view: (view * scale).into(),
                            proj: proj.into(),
                        };

                        let subbuffer = uniform_buffer.allocate_sized().unwrap();
                        *subbuffer.write().unwrap() = uniform_data;

                        subbuffer
                    };

                    let layout = pipeline.layout().set_layouts().get(0).unwrap();
                    let set = PersistentDescriptorSet::new(
                        &descriptor_set_allocator,
                        layout.clone(),
                        [WriteDescriptorSet::buffer(0, uniform_buffer_subbuffer)],
                        [],
                    )
                    .unwrap();

                    let (image_index, suboptimal, acquire_future) = match acquire_next_image(
                        swapchain.clone(),
                        None,
                    )
                    .map_err(Validated::unwrap)
                    {
                        Ok(r) => r,
                        Err(VulkanError::OutOfDate) => {
                            recreate_swapchain = true;
                            return;
                        }
                        Err(e) => panic!("failed to acquire next image: {e}"),
                    };

                    if suboptimal {
                        recreate_swapchain = true;
                    }

                    let mut builder = AutoCommandBufferBuilder::primary(
                        &command_buffer_allocator,
                        queue.queue_family_index(),
                        CommandBufferUsage::OneTimeSubmit,
                    )
                    .unwrap();
                    builder
                        .begin_render_pass(
                            RenderPassBeginInfo {
                                clear_values: vec![
                                    Some([0.0, 0.0, 1.0, 1.0].into()),
                                    Some(1f32.into()),
                                ],
                                ..RenderPassBeginInfo::framebuffer(
                                    framebuffers[image_index as usize].clone(),
                                )
                            },
                            Default::default(),
                        )
                        .unwrap()
                        .bind_pipeline_graphics(pipeline.clone())
                        .unwrap()
                        .bind_descriptor_sets(
                            PipelineBindPoint::Graphics,
                            pipeline.layout().clone(),
                            0,
                            set,
                        )
                        .unwrap()
                        .bind_vertex_buffers(0, (vertex_buffer.clone(), normals_buffer.clone()))
                        .unwrap()
                        .bind_index_buffer(index_buffer.clone())
                        .unwrap()
                        .draw_indexed(index_buffer.len() as u32, 1, 0, 0, 0)
                        .unwrap()
                        .bind_vertex_buffers(0, (vertex_buffer.clone(), normals_buffer.clone()))
                        .unwrap()
                        .bind_index_buffer(index_buffer.clone())
                        .unwrap()
                        .draw_indexed(index_buffer.len() as u32, 1, 0, 0, 0)
                        .unwrap()
                        .end_render_pass(Default::default())
                        .unwrap();
                    let command_buffer = builder.build().unwrap();

                    let future = previous_frame_end
                        .take()
                        .unwrap()
                        .join(acquire_future)
                        .then_execute(queue.clone(), command_buffer)
                        .unwrap()
                        .then_swapchain_present(
                            queue.clone(),
                            SwapchainPresentInfo::swapchain_image_index(
                                swapchain.clone(),
                                image_index,
                            ),
                        )
                        .then_signal_fence_and_flush();

                    match future.map_err(Validated::unwrap) {
                        Ok(future) => {
                            previous_frame_end = Some(future.boxed());
                        }
                        Err(VulkanError::OutOfDate) => {
                            recreate_swapchain = true;
                            previous_frame_end = Some(sync::now(device.clone()).boxed());
                        }
                        Err(e) => {
                            println!("failed to flush future: {e}");
                            previous_frame_end = Some(sync::now(device.clone()).boxed());
                        }
                    }
                    window.request_redraw();
                }
                _ => (),
            }
        }
    });
}

/// This function is called once during initialization, then again whenever the window is resized.
fn window_size_dependent_setup(
    memory_allocator: Arc<StandardMemoryAllocator>,
    vs: EntryPoint,
    fs: EntryPoint,
    images: &[Arc<Image>],
    render_pass: Arc<RenderPass>,
) -> (Arc<GraphicsPipeline>, Vec<Arc<Framebuffer>>) {
    let device = memory_allocator.device().clone();
    let extent = images[0].extent();

    let depth_buffer = ImageView::new_default(
        Image::new(
            memory_allocator,
            ImageCreateInfo {
                image_type: ImageType::Dim2d,
                format: Format::D16_UNORM,
                extent: images[0].extent(),
                usage: ImageUsage::DEPTH_STENCIL_ATTACHMENT | ImageUsage::TRANSIENT_ATTACHMENT,
                ..Default::default()
            },
            AllocationCreateInfo::default(),
        )
        .unwrap(),
    )
    .unwrap();

    let framebuffers = images
        .iter()
        .map(|image| {
            let view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view, depth_buffer.clone()],
                    ..Default::default()
                },
            )
            .unwrap()
        })
        .collect::<Vec<_>>();

    // In the triangle example we use a dynamic viewport, as its a simple example. However in the
    // teapot example, we recreate the pipelines with a hardcoded viewport instead. This allows the
    // driver to optimize things, at the cost of slower window resizes.
    // https://computergraphics.stackexchange.com/questions/5742/vulkan-best-way-of-updating-pipeline-viewport
    let pipeline = {
        let vertex_input_state = [Position::per_vertex(), Normal::per_vertex()]
            .definition(&vs.info().input_interface)
            .unwrap();
        let stages = [
            PipelineShaderStageCreateInfo::new(vs),
            PipelineShaderStageCreateInfo::new(fs),
        ];
        let layout = PipelineLayout::new(
            device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
                .into_pipeline_layout_create_info(device.clone())
                .unwrap(),
        )
        .unwrap();
        let subpass = Subpass::from(render_pass, 0).unwrap();

        GraphicsPipeline::new(
            device,
            None,
            GraphicsPipelineCreateInfo {
                stages: stages.into_iter().collect(),
                vertex_input_state: Some(vertex_input_state),
                input_assembly_state: Some(InputAssemblyState::default()),
                viewport_state: Some(ViewportState {
                    viewports: [Viewport {
                        offset: [0.0, 0.0],
                        extent: [extent[0] as f32, extent[1] as f32],
                        depth_range: 0.0..=1.0,
                    }]
                    .into_iter()
                    .collect(),
                    ..Default::default()
                }),
                rasterization_state: Some(RasterizationState::default()),
                depth_stencil_state: Some(DepthStencilState {
                    depth: Some(DepthState::simple()),
                    ..Default::default()
                }),
                multisample_state: Some(MultisampleState::default()),
                color_blend_state: Some(ColorBlendState::with_attachment_states(
                    subpass.num_color_attachments(),
                    ColorBlendAttachmentState::default(),
                )),
                subpass: Some(subpass.into()),
                ..GraphicsPipelineCreateInfo::layout(layout)
            },
        )
        .unwrap()
    };

    (pipeline, framebuffers)
}

fn load_buffers_short(
    stone: Stone,
    memory_allocator: Arc<StandardMemoryAllocator>,
    //) -> (u32, u32, u32) {
) -> (Subbuffer<[Position]>, Subbuffer<[Normal]>, Subbuffer<[u32]>) {
    let vertex_buffer = Buffer::from_iter(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::VERTEX_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        stone.positions,
    )
    .unwrap();
    let normals_buffer = Buffer::from_iter(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::VERTEX_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        stone.normals,
    )
    .unwrap();
    let index_buffer = Buffer::from_iter(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::INDEX_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        stone.indices,
    )
    .unwrap();

    return (vertex_buffer, normals_buffer, index_buffer);
}

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "./src/vert.glsl",
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "./src/frag.glsl",
    }
}
