// File automatically generated by build.rs.
// Changes made to this file will not be saved.
#[repr(C)]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    bytemuck::Pod,
    bytemuck::Zeroable,
    encase::ShaderType,
    serde::Serialize,
    serde::Deserialize
)]
pub struct UniformData {
    pub ambient: [f32; 4],
    pub view: [[f32; 4]; 4],
    pub view_proj: [[f32; 4]; 4],
    pub object_transform: [[f32; 4]; 4],
}
const _: () = assert!(
    std::mem::size_of:: < UniformData > () == 208,
    "size of UniformData does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(UniformData, ambient) == 0,
    "offset of UniformData.ambient does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(UniformData, view) == 16,
    "offset of UniformData.view does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(UniformData, view_proj) == 80,
    "offset of UniformData.view_proj does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(UniformData, object_transform) == 144,
    "offset of UniformData.object_transform does not match WGSL"
);
#[repr(C)]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    bytemuck::Pod,
    bytemuck::Zeroable,
    encase::ShaderType,
    serde::Serialize,
    serde::Deserialize
)]
pub struct DirectionalLight {
    pub color: [f32; 3],
    pub padding: f32,
    pub direction: [f32; 3],
    pub padding2: f32,
}
const _: () = assert!(
    std::mem::size_of:: < DirectionalLight > () == 32,
    "size of DirectionalLight does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(DirectionalLight, color) == 0,
    "offset of DirectionalLight.color does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(DirectionalLight, padding) == 12,
    "offset of DirectionalLight.padding does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(DirectionalLight, direction) == 16,
    "offset of DirectionalLight.direction does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(DirectionalLight, padding2) == 28,
    "offset of DirectionalLight.padding2 does not match WGSL"
);
#[repr(C)]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    bytemuck::Pod,
    bytemuck::Zeroable,
    encase::ShaderType,
    serde::Serialize,
    serde::Deserialize
)]
pub struct PointLight {
    pub color: [f32; 3],
    pub position: [f32; 3],
}
#[repr(C)]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    bytemuck::Pod,
    bytemuck::Zeroable,
    encase::ShaderType,
    serde::Serialize,
    serde::Deserialize
)]
pub struct PixelData {
    pub albedo: [f32; 4],
    pub diffuse_color: [f32; 3],
    pub roughness: f32,
    pub normal: [f32; 3],
    pub metallic: f32,
    pub emissive: [f32; 3],
    pub reflectance: f32,
    pub f0: [f32; 3],
    pub material_flags: u32,
}
#[repr(C)]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    bytemuck::Pod,
    bytemuck::Zeroable,
    encase::ShaderType,
    serde::Serialize,
    serde::Deserialize
)]
pub struct MaterialData {
    pub albedo: [f32; 4],
    pub emissive: [f32; 3],
    pub roughness: f32,
    pub metallic: f32,
    pub reflectance: f32,
    pub flags: u32,
    pub texture_enable: u32,
}
const _: () = assert!(
    std::mem::size_of:: < MaterialData > () == 48,
    "size of MaterialData does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(MaterialData, albedo) == 0,
    "offset of MaterialData.albedo does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(MaterialData, emissive) == 16,
    "offset of MaterialData.emissive does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(MaterialData, roughness) == 28,
    "offset of MaterialData.roughness does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(MaterialData, metallic) == 32,
    "offset of MaterialData.metallic does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(MaterialData, reflectance) == 36,
    "offset of MaterialData.reflectance does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(MaterialData, flags) == 40,
    "offset of MaterialData.flags does not match WGSL"
);
const _: () = assert!(
    memoffset::offset_of!(MaterialData, texture_enable) == 44,
    "offset of MaterialData.texture_enable does not match WGSL"
);
#[repr(C)]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    bytemuck::Pod,
    bytemuck::Zeroable,
    encase::ShaderType,
    serde::Serialize,
    serde::Deserialize
)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tangent: [f32; 4],
    pub tex_coords: [f32; 2],
}
pub mod bind_groups {
    pub struct BindGroup0(wgpu::BindGroup);
    pub struct BindGroupLayout0<'a> {
        pub primary_sampler: &'a wgpu::Sampler,
        pub uniforms: wgpu::BufferBinding<'a>,
        pub directional_lights: wgpu::BufferBinding<'a>,
    }
    const LAYOUT_DESCRIPTOR0: wgpu::BindGroupLayoutDescriptor = wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage {
                        read_only: true,
                    },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    };
    impl BindGroup0 {
        pub fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
            device.create_bind_group_layout(&LAYOUT_DESCRIPTOR0)
        }
        pub fn from_bindings(device: &wgpu::Device, bindings: BindGroupLayout0) -> Self {
            let bind_group_layout = device.create_bind_group_layout(&LAYOUT_DESCRIPTOR0);
            let bind_group = device
                .create_bind_group(
                    &wgpu::BindGroupDescriptor {
                        layout: &bind_group_layout,
                        entries: &[
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: wgpu::BindingResource::Sampler(
                                    bindings.primary_sampler,
                                ),
                            },
                            wgpu::BindGroupEntry {
                                binding: 1,
                                resource: wgpu::BindingResource::Buffer(bindings.uniforms),
                            },
                            wgpu::BindGroupEntry {
                                binding: 2,
                                resource: wgpu::BindingResource::Buffer(
                                    bindings.directional_lights,
                                ),
                            },
                        ],
                        label: None,
                    },
                );
            Self(bind_group)
        }
        pub fn set<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
            render_pass.set_bind_group(0, &self.0, &[]);
        }
    }
    pub struct BindGroup1(wgpu::BindGroup);
    pub struct BindGroupLayout1<'a> {
        pub roughness_tex: &'a wgpu::TextureView,
        pub albedo_tex: &'a wgpu::TextureView,
        pub normal_tex: &'a wgpu::TextureView,
        pub metallic_tex: &'a wgpu::TextureView,
        pub emissive_tex: &'a wgpu::TextureView,
        pub material_uniform: wgpu::BufferBinding<'a>,
    }
    const LAYOUT_DESCRIPTOR1: wgpu::BindGroupLayoutDescriptor = wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 3,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float {
                        filterable: true,
                    },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float {
                        filterable: true,
                    },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float {
                        filterable: true,
                    },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 4,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float {
                        filterable: true,
                    },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 5,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float {
                        filterable: true,
                    },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    };
    impl BindGroup1 {
        pub fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
            device.create_bind_group_layout(&LAYOUT_DESCRIPTOR1)
        }
        pub fn from_bindings(device: &wgpu::Device, bindings: BindGroupLayout1) -> Self {
            let bind_group_layout = device.create_bind_group_layout(&LAYOUT_DESCRIPTOR1);
            let bind_group = device
                .create_bind_group(
                    &wgpu::BindGroupDescriptor {
                        layout: &bind_group_layout,
                        entries: &[
                            wgpu::BindGroupEntry {
                                binding: 3,
                                resource: wgpu::BindingResource::TextureView(
                                    bindings.roughness_tex,
                                ),
                            },
                            wgpu::BindGroupEntry {
                                binding: 1,
                                resource: wgpu::BindingResource::TextureView(
                                    bindings.albedo_tex,
                                ),
                            },
                            wgpu::BindGroupEntry {
                                binding: 2,
                                resource: wgpu::BindingResource::TextureView(
                                    bindings.normal_tex,
                                ),
                            },
                            wgpu::BindGroupEntry {
                                binding: 4,
                                resource: wgpu::BindingResource::TextureView(
                                    bindings.metallic_tex,
                                ),
                            },
                            wgpu::BindGroupEntry {
                                binding: 5,
                                resource: wgpu::BindingResource::TextureView(
                                    bindings.emissive_tex,
                                ),
                            },
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: wgpu::BindingResource::Buffer(
                                    bindings.material_uniform,
                                ),
                            },
                        ],
                        label: None,
                    },
                );
            Self(bind_group)
        }
        pub fn set<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
            render_pass.set_bind_group(1, &self.0, &[]);
        }
    }
    pub struct BindGroups<'a> {
        pub bind_group0: &'a BindGroup0,
        pub bind_group1: &'a BindGroup1,
    }
    pub fn set_bind_groups<'a>(
        pass: &mut wgpu::RenderPass<'a>,
        bind_groups: BindGroups<'a>,
    ) {
        bind_groups.bind_group0.set(pass);
        bind_groups.bind_group1.set(pass);
    }
}
pub mod vertex {
    impl super::Vertex {
        pub const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; 4] = [
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: memoffset::offset_of!(super::Vertex, position) as u64,
                shader_location: 0,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: memoffset::offset_of!(super::Vertex, normal) as u64,
                shader_location: 1,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x4,
                offset: memoffset::offset_of!(super::Vertex, tangent) as u64,
                shader_location: 2,
            },
            wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x2,
                offset: memoffset::offset_of!(super::Vertex, tex_coords) as u64,
                shader_location: 3,
            },
        ];
        pub const fn vertex_buffer_layout(
            step_mode: wgpu::VertexStepMode,
        ) -> wgpu::VertexBufferLayout<'static> {
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<super::Vertex>() as u64,
                step_mode,
                attributes: &super::Vertex::VERTEX_ATTRIBUTES,
            }
        }
    }
}
pub const ENTRY_VS_MAIN: &str = "vs_main";
pub const ENTRY_FS_MAIN: &str = "fs_main";
pub struct VertexEntry<const N: usize> {
    entry_point: &'static str,
    buffers: [wgpu::VertexBufferLayout<'static>; N],
}
pub fn vertex_state<'a, const N: usize>(
    module: &'a wgpu::ShaderModule,
    entry: &'a VertexEntry<N>,
) -> wgpu::VertexState<'a> {
    wgpu::VertexState {
        module,
        entry_point: entry.entry_point,
        buffers: &entry.buffers,
    }
}
pub fn vs_main_entry(vertex: wgpu::VertexStepMode) -> VertexEntry<1> {
    VertexEntry {
        entry_point: ENTRY_VS_MAIN,
        buffers: [Vertex::vertex_buffer_layout(vertex)],
    }
}
pub fn create_shader_module(device: &wgpu::Device) -> wgpu::ShaderModule {
    let source = std::borrow::Cow::Borrowed(include_str!("../shaders/shader.wgsl"));
    device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(source),
        })
}
pub fn create_pipeline_layout(device: &wgpu::Device) -> wgpu::PipelineLayout {
    device
        .create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[
                    &bind_groups::BindGroup0::get_bind_group_layout(device),
                    &bind_groups::BindGroup1::get_bind_group_layout(device),
                ],
                push_constant_ranges: &[],
            },
        )
}
