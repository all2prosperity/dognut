use std::iter;

use std::num::NonZeroU32;
use cgmath::prelude::*;
use pixels::wgpu::util::DeviceExt;
use pixels::wgpu;
use winit::{
    event::*,

};


use std::time::{Duration};
use crate::department::types::msg::TransferMsg;
use crate::department::types::multi_sender::MultiSender;
use crate::department::common::constant::{WIDTH, HEIGHT};
use crossbeam_channel::Receiver;
use lazy_static::lazy_static;
use log::info;
use winit::dpi::{LogicalSize, PhysicalSize};
use crate::department::view::camera_trait;
use crate::department::preview::{position, vector};
use crate::department::view::camera as dn_camera;


use super::model;
use super::texture;
use super::resources;

use model::{DrawModel, Vertex};

use crate::wgpu::create_render_pipeline;
use crate::wgpu::instance::{Instance, InstanceRaw};
use crate::wgpu::light::LightUniform;


use crate::util::{ARG};
use crate::department::control::camera_controller::CameraController;


const NUM_INSTANCES_PER_ROW: u32 = 10;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_position: [f32; 4],
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    fn new() -> Self {
        Self {
            view_position: [0.; 4],
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    fn update_view_proj<T: camera_trait::CameraTrait>(&mut self, camera: &T) {
        let old = self.view_position.clone();
        self.view_position = camera.to_view_position();
        if old != self.view_position {
            //println!("new pos is {:?}", &self.view_position);
        }
        self.view_proj = camera.to_view_proj();
    }
}


pub struct State<T> where T: camera_trait::CameraTrait {
    tui_size: (u32, u32),
    device: wgpu::Device,
    queue: wgpu::Queue,
    render_pipeline: wgpu::RenderPipeline,
    obj_model: model::Model,
    light_model: model::Model,
    camera: T,
    pub camera_controller: CameraController,
    camera_uniform: CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    instances: Vec<Instance>,
    #[allow(dead_code)]
    instance_buffer: wgpu::Buffer,
    depth_texture: texture::Texture,
    tui_depth_texture: texture::Texture,
    size: LogicalSize<u32>,
    light_uniform: LightUniform,
    light_buffer: wgpu::Buffer,
    light_bind_group: wgpu::BindGroup,
    light_render_pipeline: wgpu::RenderPipeline,
    pub mouse_pressed: bool,
    pub scale_factor: f64,
    light_degree: u32,
}

impl<T> State<T> where T: camera_trait::CameraTrait {
    pub async fn new(size: LogicalSize<u32>, camera: T) -> Self {
        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        log::warn!("WGPU setup");
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        log::warn!("device and queue");
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    limits: wgpu::Limits::default(),
                },
                // Some(&std::path::Path::new("trace")), // Trace path
                None, // Trace path
            )
            .await
            .unwrap();
        
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });
        //let camera = camera::Camera::new((0.0, 0., 10.), cgmath::Deg(-90.0), cgmath::Deg(-0.0));
        //let projection = camera::Projection::new(size.width, size.height, cgmath::Deg(45.), 0.1, 100.0);
        let camera_controller = CameraController::new(2.0, 0.2, false);

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        const SPACE_BETWEEN: f32 = 3.0;
        
        let position = cgmath::Vector3 { x:0., y: 0., z: -5. };

        let rotation = cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(0.0));
        let instances = vec![Instance{position, rotation}];

        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        println!("instance {:?}", instance_data.len());
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        log::warn!("Load model");
        let obj_model = resources::load_model(
            &ARG.obj_path,
            &device,
            &queue,
            &texture_bind_group_layout,
        ).await.unwrap();

        let light_model = resources::load_model(
            "./res/nice_cube/light_ball.obj",
            &device,
            &queue,
            &texture_bind_group_layout,
        ).await.unwrap();

        let depth_texture =
            texture::Texture::create_depth_texture(&device, (size.width, size.height), "depth_texture");

        let tui_depth_texture = texture::Texture::create_depth_texture(&device, (256, 79), "tui_depth_texture");

        let light_uniform = LightUniform::default();

        let light_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("light"),
                contents: bytemuck::cast_slice(&[light_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );
        let light_bind_group_layout = LightUniform::bind_group_layout(&device);

        let light_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: None,
            layout: &light_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding:0,
                    resource: light_buffer.as_entire_binding(),
                }
            ],
        });

        let light_render_pipeline = {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Light Pipeline Layout"),
                bind_group_layouts: &[&camera_bind_group_layout, &light_bind_group_layout],
                push_constant_ranges: &[],
            });
            let shader = wgpu::ShaderModuleDescriptor {
                label: Some("Light Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("../../res/shaders/light.wgsl").into()),
            };
            create_render_pipeline(
                &device,
                &layout,
                wgpu::TextureFormat::Rgba8UnormSrgb,
                Some(texture::Texture::DEPTH_FORMAT),
                &[model::ModelVertex::desc()],
                shader,
                "light_pipeline"
            )
        };

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout, &camera_bind_group_layout, &light_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = {
            let shader = wgpu::ShaderModuleDescriptor {
                label: Some("Normal shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("../../res/shaders/shader.wgsl").into()),
            };
            create_render_pipeline(
                &device, &render_pipeline_layout,
                wgpu::TextureFormat::Rgba8UnormSrgb,
                Some(texture::Texture::DEPTH_FORMAT),
                &[model::ModelVertex::desc(), InstanceRaw::desc()],
                shader,
                "render_pipeline"
            )
        };

        Self {
            tui_size: (256, 79),
            device,
            queue,
            render_pipeline,
            obj_model,
            camera,
            camera_controller,
            camera_buffer,
            camera_bind_group,
            camera_uniform,
            instances,
            instance_buffer,
            depth_texture,
            tui_depth_texture,
            size,
            light_model,
            light_uniform,
            light_buffer,
            light_bind_group,
            light_render_pipeline,
            mouse_pressed: false,
            scale_factor: 1.0f64,
            light_degree: 0,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>, scale_factor: f64) {
        let logical_size = new_size.to_logical::<u32>(scale_factor);
        if logical_size.width % 256 != 0 {
            return ;
        }
        if logical_size.width > 0 && logical_size.height > 0 {
            
            info!("window update from {:?} to {:?}", self.size, logical_size);
            self.camera.update_projection(logical_size.width, logical_size.height);
            self.size = logical_size;
            self.depth_texture =
                texture::Texture::create_depth_texture(&self.device,(self.size.width, self.size.height), "depth_label");
        }
    }

    pub fn input(&mut self, event: &DeviceEvent) -> bool {
        match event {
            DeviceEvent::Key(
                KeyboardInput {
                    virtual_keycode: Some(key),
                    state,
                    ..
                }
            ) => self.camera_controller.process_keyboard(*key, *state),
            DeviceEvent::MouseWheel { delta, .. } => {
                self.camera_controller.process_scroll(delta);
                true
            }
            DeviceEvent::Button {
                button: 0, // Left Mouse Button
                state,
            } => {
                self.mouse_pressed = *state == ElementState::Pressed;
                true
            }
            DeviceEvent::MouseMotion { delta } => {
                if self.mouse_pressed {
                    self.camera_controller.process_mouse(delta.0, delta.1);
                }
                true
            }
            _ => false,
        }
    }

    pub fn update(&mut self, dt: Duration) {
        self.camera_controller.update_camera(&mut self.camera, dt);
        self.camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );

        let data = self.camera_controller.model_ctrl.update_model(dt);
        self.queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(&data)
        );

        let old_position: cgmath::Vector3<_> = self.light_uniform.position.into();
        self.light_uniform.position =
            (cgmath::Quaternion::from_axis_angle((0.0, 1.0, 0.0).into(), cgmath::Deg(1.0))
                * old_position)
                .into();
        self.queue.write_buffer(&self.light_buffer, 0, bytemuck::cast_slice(&[self.light_uniform]));

    }

    pub fn update_outside(&mut self, controller:&mut CameraController,dt: Duration) {
        controller.update_camera(&mut self.camera, dt);
        self.camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );

        let data = controller.model_ctrl.update_model(dt);
        self.queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(&data)
        );

        let old_position: cgmath::Vector3<_> = self.light_uniform.position.into();

        // self.light_degree += 1;
        // let res = self.light_degree / 180;
        // let mut clockwise = true;
        // if res % 2 == 1 {
        //     clockwise = false;
        // }
        //
        // self.light_uniform.position =
        //     (cgmath::Quaternion::from_axis_angle((0.0, 1.0, 0.0).into(), cgmath::Deg(if clockwise {-1.0} else {1.0}))
        //         * old_position)
        //         .into();
        // self.queue.write_buffer(&self.light_buffer, 0, bytemuck::cast_slice(&[self.light_uniform]));

    }

    // the first return Vec is for gui, the second is for tui
    pub fn render(&mut self, tui_with_window: bool) -> (Vec<u8>, Option<Vec<u8>>) {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let (texture_desc, texture) = self.encode_a_new_render_texutre(&mut encoder, (self.size.width, self.size.height), &self.depth_texture);
        let u32_size = std::mem::size_of::<u32>() as u32;
        let output_buffer = self.create_output_buffer((self.size.width, self.size.height),u32_size);

        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::ImageCopyBuffer {
                buffer: &output_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: NonZeroU32::new(u32_size * self.size.width),
                    rows_per_image: NonZeroU32::new(self.size.height),
                },
            },
            texture_desc.size,
        );

        let tui_output_buffer = self.create_output_buffer(self.tui_size, u32_size);
        if tui_with_window {
            let (tui_desc, tui_texture) = self.encode_a_new_render_texutre(&mut encoder, (self.tui_size.0, self.tui_size.1), &self.tui_depth_texture);


            encoder.copy_texture_to_buffer(
                wgpu::ImageCopyTexture {
                    aspect: wgpu::TextureAspect::All,
                    texture: &tui_texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                },
                wgpu::ImageCopyBuffer {
                    buffer: &tui_output_buffer,
                    layout: wgpu::ImageDataLayout {
                        offset: 0,
                        bytes_per_row: NonZeroU32::new(u32_size * self.tui_size.0),
                        rows_per_image: NonZeroU32::new(self.tui_size.1),
                    },
                },
                tui_desc.size,
            );
        }

        self.queue.submit(iter::once(encoder.finish()));

        let mut ret_buf = Vec::new();
        let mut tui_buf = None;
        {
            let buffer_slice = output_buffer.slice(..);
            // NOTE: We have to create the mapping THEN device.poll() before await
            // the future. Otherwise the application will freeze.
            let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
            buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
                tx.send(result).unwrap();
            });

            let mut tui_slice = None;

            if tui_with_window {
                tui_slice = Some(tui_output_buffer.slice(..));
                // tui_slice will be mapped with buffer_slice, so we don't need to send a signal.
                tui_slice.unwrap().map_async(wgpu::MapMode::Read, move |_result| {
                });
            }
            self.device.poll(wgpu::Maintain::Wait);
            pollster::block_on(rx.receive());

            let data = buffer_slice.get_mapped_range();
            ret_buf = data.iter().map(|x| *x).collect();
            if tui_with_window {
                let data = tui_slice.unwrap().get_mapped_range();
                tui_buf = Some(data.iter().map(|x| *x).collect());
            }
        }
        output_buffer.unmap();
        (ret_buf, tui_buf)
    }

    fn create_output_buffer(&self, size: (u32, u32), u32_size: u32) -> wgpu::Buffer {
        let output_buffer_size = (u32_size * size.0 * size.1) as wgpu::BufferAddress;
        let output_buffer_desc = wgpu::BufferDescriptor {
            size: output_buffer_size,
            usage: wgpu::BufferUsages::COPY_DST
                // this tells wpgu that we want to read this buffer from the cpu
                | wgpu::BufferUsages::MAP_READ,
            label: None,
            mapped_at_creation: false,
        };
        let output_buffer = self.device.create_buffer(&output_buffer_desc);
        output_buffer
    }

    fn encode_a_new_render_texutre(&self, encoder: &mut wgpu::CommandEncoder, w_h: (u32, u32), depth_texture: &texture::Texture) -> (wgpu::TextureDescriptor, wgpu::Texture) {
        let texture_desc = wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: w_h.0,
                height: w_h.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
        };
        let texture = self.device.create_texture(&texture_desc);
        let view = texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            use crate::wgpu::model::DrawLight;
            render_pass.set_pipeline(&self.light_render_pipeline);
            render_pass.draw_light_model(&self.light_model, &self.camera_bind_group, &self.light_bind_group);

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw_model_instanced(
                &self.obj_model,
                0..self.instances.len() as u32,
                &self.camera_bind_group,
                &self.light_bind_group
            );
        }
        (texture_desc, texture)
    }
}

pub fn run(_r: Receiver<TransferMsg>, ms: MultiSender<TransferMsg>) {
    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    rt.block_on(async {

        let camera = dn_camera::Camera::new(
            45.,
            WIDTH as f32 / HEIGHT as f32,
            0.1,
            100.,
            position::Pos3::from_xyz(0.0, 0., 10.),
            vector::Vector3::from_xyz(0., 0., 1.),
            vector::Vector3::from_xyz(0., 1., 0.),
        );
        println!("use new camera");
        // let projection = cg_camera::Projection::new(WIDTH, HEIGHT, cgmath::Deg(45.), 0.1, 100.0);
        // let camera = cg_camera::Camera::new((0.0, 0., 10.), cgmath::Deg(-90.0), cgmath::Deg(-0.0), projection);
        let mut state = State::new(LogicalSize{height: HEIGHT, width:WIDTH}, camera).await;
        loop {
            let buf = state.render(false).0;
            ms.net.send(TransferMsg::RenderedData(buf.clone()));
            ms.win.send(TransferMsg::RenderedData(buf));
            // println!("render once");
            // sleep(Duration::from_millis(100)).await
        }
    });
}
