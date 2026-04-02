use windows::Win32::Graphics::Direct3D12::{
    D3D12_CPU_PAGE_PROPERTY_UNKNOWN, D3D12_HEAP_PROPERTIES, D3D12_HEAP_TYPE_UPLOAD,
    D3D12_MEMORY_POOL_UNKNOWN, D3D12_RESOURCE_DESC, D3D12_RESOURCE_DIMENSION_BUFFER,
    D3D12_RESOURCE_STATE_GENERIC_READ, D3D12_TEXTURE_LAYOUT_ROW_MAJOR, D3D12_VERTEX_BUFFER_VIEW,
    ID3D12Device, ID3D12GraphicsCommandList, ID3D12Resource,
};
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT_UNKNOWN, DXGI_SAMPLE_DESC};

struct Vertex {
    position: [f32; 3],
    color: [f32; 4],
}

struct VertexResource {
    vertex_buffer: ID3D12Resource,
    vertex_buffer_view: D3D12_VERTEX_BUFFER_VIEW,
}

impl VertexResource {
    pub fn new(device: &ID3D12Device) -> Self {
        let vertices = [
            Vertex {
                position: [0.0, 0.5, 0.0],
                color: [1.0, 0.0, 0.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                color: [0.0, 1.0, 0.0, 1.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.0],
                color: [0.0, 0.0, 1.0, 1.0],
            },
        ];

        let vertex_buffer_size = (size_of::<Vertex>() * vertices.len()) as u64;

        let heap_properties = D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_UPLOAD,
            CPUPageProperty: D3D12_CPU_PAGE_PROPERTY_UNKNOWN,
            MemoryPoolPreference: D3D12_MEMORY_POOL_UNKNOWN,
            CreationNodeMask: 0,
            VisibleNodeMask: 0,
        };
        let resource_desc = D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
            Alignment: 0,
            Width: vertex_buffer_size,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: DXGI_FORMAT_UNKNOWN,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
            Flags: Default::default(),
        };

        let mut vertex_buffer: Option<ID3D12Resource> = None;
        match unsafe {
            device.CreateCommittedResource(
                &heap_properties,
                Default::default(),
                &resource_desc,
                D3D12_RESOURCE_STATE_GENERIC_READ,
                None,
                &mut vertex_buffer,
            )
        } {
            Ok(_) => (),
            Err(hr) => panic!("Failed to create vertex buffer resource: {:?}", hr),
        }
        let vertex_buffer = vertex_buffer.expect("Vertex buffer resource was not created");

        unsafe {
            let mut data = std::ptr::null_mut();
            match vertex_buffer.Map(0, None, Some(&mut data)) {
                Ok(_) => {
                    std::ptr::copy_nonoverlapping(
                        vertices.as_ptr(),
                        data as *mut Vertex,
                        vertices.len(),
                    );
                    vertex_buffer.Unmap(0, None);
                }
                Err(hr) => panic!("Failed to map vertex buffer resource: {:?}", hr),
            }
        }

        let vertex_buffer_view = D3D12_VERTEX_BUFFER_VIEW {
            BufferLocation: unsafe { vertex_buffer.GetGPUVirtualAddress() },
            SizeInBytes: vertex_buffer_size as u32,
            StrideInBytes: size_of::<Vertex>() as u32,
        };

        Self {
            vertex_buffer,
            vertex_buffer_view,
        }
    }

    pub fn record_draw_commands(&self, command_list: &ID3D12GraphicsCommandList) {
        unsafe {
            command_list.IASetVertexBuffers(0, Some(&[self.vertex_buffer_view]));
        }
    }
}
