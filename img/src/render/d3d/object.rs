use crate::img::{Image, Pixel};
use windows::Win32::Graphics::Direct3D12::{
    D3D12_CPU_PAGE_PROPERTY_UNKNOWN, D3D12_CPU_PAGE_PROPERTY_WRITE_BACK,
    D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING, D3D12_DESCRIPTOR_HEAP_DESC,
    D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE, D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV,
    D3D12_HEAP_PROPERTIES, D3D12_HEAP_TYPE_CUSTOM, D3D12_HEAP_TYPE_UPLOAD, D3D12_INDEX_BUFFER_VIEW,
    D3D12_MEMORY_POOL_L0, D3D12_MEMORY_POOL_UNKNOWN, D3D12_RESOURCE_DESC,
    D3D12_RESOURCE_DIMENSION_BUFFER, D3D12_RESOURCE_DIMENSION_TEXTURE2D, D3D12_RESOURCE_FLAG_NONE,
    D3D12_RESOURCE_STATE_GENERIC_READ, D3D12_SHADER_RESOURCE_VIEW_DESC,
    D3D12_SHADER_RESOURCE_VIEW_DESC_0, D3D12_SRV_DIMENSION_TEXTURE2D, D3D12_TEX2D_SRV,
    D3D12_TEXTURE_LAYOUT_ROW_MAJOR, D3D12_TEXTURE_LAYOUT_UNKNOWN, D3D12_VERTEX_BUFFER_VIEW,
    ID3D12DescriptorHeap, ID3D12Device, ID3D12GraphicsCommandList, ID3D12Resource,
};
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT_R16_UINT, DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_FORMAT_UNKNOWN, DXGI_SAMPLE_DESC};

struct Vertex {
    position: [f32; 3],
    uv: [f32; 2],
}

pub struct Object {
    vertex_buffer: ID3D12Resource,
    vertex_buffer_view: D3D12_VERTEX_BUFFER_VIEW,
    index_buffer: ID3D12Resource,
    index_buffer_view: D3D12_INDEX_BUFFER_VIEW,
    texture: Option<ID3D12Resource>,
    desc_heap: ID3D12DescriptorHeap,
}

impl Object {
    const VERTICES: [Vertex; 4] = [
        Vertex {
            position: [-1.0, -1.0, 0.0],
            uv: [0.0, 1.0],
        },
        Vertex {
            position: [-1.0, 1.0, 0.0],
            uv: [0.0, 0.0],
        },
        Vertex {
            position: [1.0, -1.0, 0.0],
            uv: [1.0, 1.0],
        },
        Vertex {
            position: [1.0, 1.0, 0.0],
            uv: [1.0, 0.0],
        },
    ];

    const INDICES: [u16; 6] = [0, 1, 2, 2, 1, 3];

    pub fn new(device: &ID3D12Device) -> Self {
        let (vertex_buffer, vertex_buffer_view) = Self::create_vertex_buffer(device);
        let (index_buffer, index_buffer_view) = Self::create_index_buffer(device);

        let descriptor_heap_desc = D3D12_DESCRIPTOR_HEAP_DESC {
            Type: D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV,
            NumDescriptors: 1,
            Flags: D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE,
            NodeMask: 0,
        };
        let desc_heap = match unsafe { device.CreateDescriptorHeap(&descriptor_heap_desc) } {
            Ok(heap) => heap,
            Err(hr) => panic!("Failed to create descriptor heap: {:?}", hr),
        };

        Self {
            vertex_buffer,
            vertex_buffer_view,
            index_buffer,
            index_buffer_view,
            texture: None,
            desc_heap,
        }
    }

    pub fn record_draw_commands(&self, command_list: &ID3D12GraphicsCommandList) {
        unsafe {
            command_list.IASetVertexBuffers(0, Some(&[self.vertex_buffer_view]));
            command_list.IASetIndexBuffer(Some(&self.index_buffer_view));

            command_list.SetDescriptorHeaps(&[Some(self.desc_heap.clone())]);
            command_list.SetGraphicsRootDescriptorTable(
                0,
                self.desc_heap.GetGPUDescriptorHandleForHeapStart(),
            );
        }
    }

    pub fn render(&self, command_list: &ID3D12GraphicsCommandList) {
        unsafe {
            command_list.DrawIndexedInstanced(Self::INDICES.len() as u32, 1, 0, 0, 0);
        }
    }

    pub fn load_image(&mut self, device: &ID3D12Device, image: &Image) {
        let heap_properties = D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_CUSTOM,
            CPUPageProperty: D3D12_CPU_PAGE_PROPERTY_WRITE_BACK,
            MemoryPoolPreference: D3D12_MEMORY_POOL_L0,
            CreationNodeMask: 0,
            VisibleNodeMask: 0,
        };
        let resource_desc = D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_TEXTURE2D,
            Alignment: 0,
            Width: image.width as u64,
            Height: image.height,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: DXGI_FORMAT_R8G8B8A8_UNORM,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Layout: D3D12_TEXTURE_LAYOUT_UNKNOWN,
            Flags: D3D12_RESOURCE_FLAG_NONE,
        };

        let mut texture: Option<ID3D12Resource> = None;
        match unsafe {
            device.CreateCommittedResource(
                &heap_properties,
                Default::default(),
                &resource_desc,
                D3D12_RESOURCE_STATE_GENERIC_READ,
                None,
                &mut texture,
            )
        } {
            Ok(_) => (),
            Err(hr) => panic!("Failed to create texture resource: {:?}", hr),
        }
        let texture = texture.expect("Texture resource was not created");

        let row_pitch = image.width * size_of::<Pixel>() as u32;
        let slice_pitch = row_pitch * image.height;

        match unsafe {
            texture.WriteToSubresource(
                0,
                None,
                image.data.as_ptr() as *const _,
                row_pitch,
                slice_pitch,
            )
        } {
            Ok(_) => self.texture = Some(texture),
            Err(hr) => panic!("Failed to write to texture resource: {:?}", hr),
        }

        let srv_desc = D3D12_SHADER_RESOURCE_VIEW_DESC {
            Format: DXGI_FORMAT_R8G8B8A8_UNORM,
            ViewDimension: D3D12_SRV_DIMENSION_TEXTURE2D,
            Shader4ComponentMapping: D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING,
            Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                Texture2D: D3D12_TEX2D_SRV {
                    MostDetailedMip: 0,
                    MipLevels: 1,
                    PlaneSlice: 0,
                    ResourceMinLODClamp: 0.0,
                },
            },
        };

        unsafe {
            device.CreateShaderResourceView(
                self.texture.as_ref().unwrap(),
                Some(&srv_desc),
                self.desc_heap.GetCPUDescriptorHandleForHeapStart(),
            );
        }
    }

    fn create_vertex_buffer(device: &ID3D12Device) -> (ID3D12Resource, D3D12_VERTEX_BUFFER_VIEW) {
        let vertex_buffer_size = (size_of::<Vertex>() * Self::VERTICES.len()) as u64;

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
                        Self::VERTICES.as_ptr(),
                        data as *mut Vertex,
                        Self::VERTICES.len(),
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

        (vertex_buffer, vertex_buffer_view)
    }

    fn create_index_buffer(device: &ID3D12Device) -> (ID3D12Resource, D3D12_INDEX_BUFFER_VIEW) {
        let index_buffer_size = (size_of::<u16>() * Self::INDICES.len()) as u64;

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
            Width: index_buffer_size,
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

        let mut index_buffer: Option<ID3D12Resource> = None;
        match unsafe {
            device.CreateCommittedResource(
                &heap_properties,
                Default::default(),
                &resource_desc,
                D3D12_RESOURCE_STATE_GENERIC_READ,
                None,
                &mut index_buffer,
            )
        } {
            Ok(_) => (),
            Err(hr) => panic!("Failed to create index buffer resource: {:?}", hr),
        }
        let index_buffer = index_buffer.expect("Index buffer resource was not created");

        unsafe {
            let mut data = std::ptr::null_mut();
            match index_buffer.Map(0, None, Some(&mut data)) {
                Ok(_) => {
                    std::ptr::copy_nonoverlapping(
                        Self::INDICES.as_ptr(),
                        data as *mut u16,
                        Self::INDICES.len(),
                    );
                    index_buffer.Unmap(0, None);
                }
                Err(hr) => panic!("Failed to map index buffer resource: {:?}", hr),
            }
        }

        let index_buffer_view = D3D12_INDEX_BUFFER_VIEW {
            BufferLocation: unsafe { index_buffer.GetGPUVirtualAddress() },
            SizeInBytes: index_buffer_size as u32,
            Format: DXGI_FORMAT_R16_UINT
        };

        (index_buffer, index_buffer_view)
    }
}
