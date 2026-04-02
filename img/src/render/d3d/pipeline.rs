use std::str::from_utf8;
use windows::core::{s, w, PCSTR, PCWSTR};
use windows::Win32::Foundation::{FALSE, TRUE};
use windows::Win32::Graphics::Direct3D::Fxc::D3DCompileFromFile;
use windows::Win32::Graphics::Direct3D::ID3DBlob;
use windows::Win32::Graphics::Direct3D12::{D3D12SerializeRootSignature, ID3D12Device, ID3D12GraphicsCommandList, ID3D12PipelineState, ID3D12RootSignature, D3D12_APPEND_ALIGNED_ELEMENT, D3D12_BLEND_DESC, D3D12_BLEND_ONE, D3D12_BLEND_OP_ADD, D3D12_BLEND_ZERO, D3D12_COLOR_WRITE_ENABLE_ALL, D3D12_CONSERVATIVE_RASTERIZATION_MODE_OFF, D3D12_CULL_MODE_NONE, D3D12_DEFAULT_DEPTH_BIAS, D3D12_DEFAULT_DEPTH_BIAS_CLAMP, D3D12_DEFAULT_SAMPLE_MASK, D3D12_DEFAULT_SLOPE_SCALED_DEPTH_BIAS, D3D12_DEPTH_STENCIL_DESC, D3D12_FILL_MODE_SOLID, D3D12_GRAPHICS_PIPELINE_STATE_DESC, D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA, D3D12_INPUT_ELEMENT_DESC, D3D12_INPUT_LAYOUT_DESC, D3D12_LOGIC_OP_NOOP, D3D12_PIPELINE_STATE_FLAG_NONE, D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE, D3D12_RASTERIZER_DESC, D3D12_RENDER_TARGET_BLEND_DESC, D3D12_ROOT_SIGNATURE_DESC, D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT, D3D12_SHADER_BYTECODE, D3D_ROOT_SIGNATURE_VERSION_1};
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT_D32_FLOAT, DXGI_FORMAT_R32G32B32A32_FLOAT, DXGI_FORMAT_R32G32B32_FLOAT, DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_FORMAT_UNKNOWN, DXGI_SAMPLE_DESC};

pub struct Pipeline {
    root_signature: ID3D12RootSignature,
    pipeline: ID3D12PipelineState,
}

impl Pipeline {
    pub fn new(device: &ID3D12Device) -> Self {
        let ps = compile_shader(w!("shader.hlsl"), s!("ps_main"), s!("ps_5_0"), 0);
        let vs = compile_shader(w!("shader.hlsl"), s!("vs_main"), s!("vs_5_0"), 0);

        let mut signature_blob: Option<ID3DBlob> = None;
        let mut error_blob: Option<ID3DBlob> = None;

        let signature_desc = D3D12_ROOT_SIGNATURE_DESC {
            NumParameters: 0,
            pParameters: std::ptr::null(),
            NumStaticSamplers: 0,
            pStaticSamplers: std::ptr::null(),
            Flags: D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
        };
        match unsafe {
            D3D12SerializeRootSignature(
                &signature_desc,
                D3D_ROOT_SIGNATURE_VERSION_1,
                &mut signature_blob,
                Some(&mut error_blob),
            )
        } {
            Ok(_) => (),
            Err(hr) => {
                if let Some(error_blob) = error_blob {
                    let error_message = unsafe {
                        from_utf8(std::slice::from_raw_parts(
                            error_blob.GetBufferPointer() as *const u8,
                            error_blob.GetBufferSize(),
                        ))
                    }
                    .unwrap_or("Failed to retrieve root signature serialization error message");

                    panic!(
                        "Failed to serialize root signature: {:?}\nError message: {}",
                        hr, error_message
                    );
                } else {
                    panic!(
                        "Failed to serialize root signature: {:?}\nNo error message available",
                        hr
                    );
                }
            }
        }

        let signature_blob = signature_blob.expect("Root signature blob was not created");
        let root_signature: ID3D12RootSignature = match unsafe {
            device.CreateRootSignature(
                0,
                std::slice::from_raw_parts(
                    signature_blob.GetBufferPointer() as *const u8,
                    signature_blob.GetBufferSize(),
                ),
            )
        } {
            Ok(root_signature) => root_signature,
            Err(hr) => panic!("Failed to create root signature: {:?}", hr),
        };

        let input_layout = [
            D3D12_INPUT_ELEMENT_DESC {
                SemanticName: s!("POSITION"),
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R32G32B32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: 0,
                InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            },
            D3D12_INPUT_ELEMENT_DESC {
                SemanticName: s!("COLOR"),
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R32G32B32A32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: D3D12_APPEND_ALIGNED_ELEMENT,
                InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            },
        ];

        let pipeline_desc = D3D12_GRAPHICS_PIPELINE_STATE_DESC {
            pRootSignature: unsafe { std::mem::transmute_copy(&root_signature) },
            VS: D3D12_SHADER_BYTECODE {
                pShaderBytecode: unsafe { vs.GetBufferPointer() },
                BytecodeLength: unsafe { vs.GetBufferSize() },
            },
            PS: D3D12_SHADER_BYTECODE {
                pShaderBytecode: unsafe { ps.GetBufferPointer() },
                BytecodeLength: unsafe { ps.GetBufferSize() },
            },
            BlendState: D3D12_BLEND_DESC {
                AlphaToCoverageEnable: FALSE,
                IndependentBlendEnable: FALSE,
                RenderTarget: [
                    D3D12_RENDER_TARGET_BLEND_DESC {
                        BlendEnable: FALSE,
                        LogicOpEnable: FALSE,
                        SrcBlend: D3D12_BLEND_ONE,
                        DestBlend: D3D12_BLEND_ZERO,
                        BlendOp: D3D12_BLEND_OP_ADD,
                        SrcBlendAlpha: D3D12_BLEND_ONE,
                        DestBlendAlpha: D3D12_BLEND_ZERO,
                        BlendOpAlpha: D3D12_BLEND_OP_ADD,
                        LogicOp: D3D12_LOGIC_OP_NOOP,
                        RenderTargetWriteMask: D3D12_COLOR_WRITE_ENABLE_ALL.0 as u8,
                    },
                    D3D12_RENDER_TARGET_BLEND_DESC::default(),
                    D3D12_RENDER_TARGET_BLEND_DESC::default(),
                    D3D12_RENDER_TARGET_BLEND_DESC::default(),
                    D3D12_RENDER_TARGET_BLEND_DESC::default(),
                    D3D12_RENDER_TARGET_BLEND_DESC::default(),
                    D3D12_RENDER_TARGET_BLEND_DESC::default(),
                    D3D12_RENDER_TARGET_BLEND_DESC::default(),
                ],
            },
            SampleMask: D3D12_DEFAULT_SAMPLE_MASK,
            RasterizerState: D3D12_RASTERIZER_DESC {
                FillMode: D3D12_FILL_MODE_SOLID,
                CullMode: D3D12_CULL_MODE_NONE,
                FrontCounterClockwise: FALSE,
                DepthBias: D3D12_DEFAULT_DEPTH_BIAS,
                DepthBiasClamp: D3D12_DEFAULT_DEPTH_BIAS_CLAMP,
                SlopeScaledDepthBias: D3D12_DEFAULT_SLOPE_SCALED_DEPTH_BIAS,
                DepthClipEnable: TRUE,
                MultisampleEnable: FALSE,
                AntialiasedLineEnable: FALSE,
                ForcedSampleCount: 0,
                ConservativeRaster: D3D12_CONSERVATIVE_RASTERIZATION_MODE_OFF,
            },
            InputLayout: D3D12_INPUT_LAYOUT_DESC {
                pInputElementDescs: input_layout.as_ptr(),
                NumElements: input_layout.len() as u32,
            },
            PrimitiveTopologyType: D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE,
            NumRenderTargets: 1,
            RTVFormats: [
                DXGI_FORMAT_R8G8B8A8_UNORM,
                DXGI_FORMAT_UNKNOWN,
                DXGI_FORMAT_UNKNOWN,
                DXGI_FORMAT_UNKNOWN,
                DXGI_FORMAT_UNKNOWN,
                DXGI_FORMAT_UNKNOWN,
                DXGI_FORMAT_UNKNOWN,
                DXGI_FORMAT_UNKNOWN,
            ],
            DSVFormat: DXGI_FORMAT_D32_FLOAT,
            DepthStencilState: D3D12_DEPTH_STENCIL_DESC::default(),
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            NodeMask: 0,
            Flags: D3D12_PIPELINE_STATE_FLAG_NONE,
            ..Default::default()
        };
        let pipeline: ID3D12PipelineState =
            match unsafe { device.CreateGraphicsPipelineState(&pipeline_desc) } {
                Ok(pipeline) => pipeline,
                Err(hr) => panic!("Failed to create graphics pipeline state: {:?}", hr),
            };

        Self {
            root_signature,
            pipeline,
        }
    }

    pub fn record_commands(&self, command_list: &ID3D12GraphicsCommandList) {
        unsafe {
            command_list.SetPipelineState(&self.pipeline);
            command_list.SetGraphicsRootSignature(&self.root_signature);
        }
    }
}

fn compile_shader(file_path: PCWSTR, entry_point: PCSTR, target: PCSTR, flags: u32) -> ID3DBlob {
    let mut shader_blob: Option<ID3DBlob> = None;
    let mut error_blob: Option<ID3DBlob> = None;

    match unsafe {
        D3DCompileFromFile(
            file_path,
            None,
            None,
            entry_point,
            target,
            flags,
            0,
            &mut shader_blob,
            Some(&mut error_blob),
        )
    } {
        Ok(_) => shader_blob.unwrap(),
        Err(hr) => {
            if let Some(error_blob) = error_blob {
                let error_message = unsafe {
                    from_utf8(std::slice::from_raw_parts(
                        error_blob.GetBufferPointer() as *const u8,
                        error_blob.GetBufferSize(),
                    ))
                }
                .unwrap_or("Failed to retrieve shader compilation error message");

                panic!(
                    "Failed to compile shader: {:?}\nError message: {}",
                    hr, error_message
                );
            } else {
                panic!(
                    "Failed to compile shader: {:?}\nNo error message available",
                    hr
                );
            }
        }
    }
}
