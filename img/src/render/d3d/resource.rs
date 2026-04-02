use crate::img::Image;
use crate::render::d3d::object::Object;
use crate::render::d3d::pipeline::Pipeline;
use windows::Win32::Foundation::{HANDLE, HWND, RECT};
use windows::Win32::Graphics::Direct3D::D3D10_PRIMITIVE_TOPOLOGY_TRIANGLELIST;
use windows::Win32::Graphics::Direct3D12::{
    D3D12_COMMAND_LIST_TYPE_DIRECT, D3D12_COMMAND_QUEUE_DESC, D3D12_COMMAND_QUEUE_PRIORITY_NORMAL,
    D3D12_DESCRIPTOR_HEAP_DESC, D3D12_DESCRIPTOR_HEAP_TYPE_RTV, D3D12_RESOURCE_BARRIER,
    D3D12_RESOURCE_BARRIER_0, D3D12_RESOURCE_BARRIER_FLAG_NONE,
    D3D12_RESOURCE_BARRIER_TYPE_TRANSITION, D3D12_RESOURCE_STATE_PRESENT,
    D3D12_RESOURCE_STATE_RENDER_TARGET, D3D12_RESOURCE_TRANSITION_BARRIER, D3D12_VIEWPORT,
    ID3D12CommandAllocator, ID3D12CommandQueue, ID3D12DescriptorHeap, ID3D12Device, ID3D12Fence,
    ID3D12GraphicsCommandList, ID3D12Resource,
};
use windows::Win32::Graphics::Dxgi::Common::{
    DXGI_ALPHA_MODE_UNSPECIFIED, DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_SAMPLE_DESC,
};
use windows::Win32::Graphics::Dxgi::{
    DXGI_SCALING_STRETCH, DXGI_SWAP_CHAIN_DESC1, DXGI_SWAP_EFFECT_FLIP_DISCARD,
    DXGI_USAGE_RENDER_TARGET_OUTPUT, IDXGIFactory7, IDXGISwapChain4,
};
use windows::Win32::System::Threading::{CreateEventW, INFINITE, WaitForSingleObject};
use windows::Win32::UI::WindowsAndMessaging::GetClientRect;
use windows::core::Interface;

pub struct Resources {
    commands: Commands,
    display: Display,
    vertex_resource: Object,
    pipeline: Pipeline,
    view_port: D3D12_VIEWPORT,
    scissor_rect: RECT,
}

impl Resources {
    pub fn new(device: &ID3D12Device, factory: &IDXGIFactory7, hwnd: &HWND) -> Self {
        let commands = Commands::new(device);
        let display = Display::new(factory, device, hwnd, &commands.command_queue);
        let vertex_resource = Object::new(device);
        let pipeline = Pipeline::new(device);

        let mut rect = RECT::default();
        unsafe {
            GetClientRect(*hwnd, &mut rect);
        }
        let view_port = D3D12_VIEWPORT {
            TopLeftX: 0.0,
            TopLeftY: 0.0,
            Width: (rect.right - rect.left) as f32,
            Height: (rect.bottom - rect.top) as f32,
            MinDepth: 0.0,
            MaxDepth: 1.0,
        };
        let scissor_rect = rect;

        Self {
            commands,
            display,
            vertex_resource,
            pipeline,
            view_port,
            scissor_rect,
        }
    }

    pub fn render(&mut self, device: &ID3D12Device) {
        let frame_index = self.display.frame_index();

        let command_list = &self.commands.command_list;
        self.display.begin_frame(device, command_list, frame_index);

        unsafe {
            command_list.RSSetViewports(&[self.view_port]);
            command_list.RSSetScissorRects(&[self.scissor_rect]);
            command_list.IASetPrimitiveTopology(D3D10_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
            self.vertex_resource.record_draw_commands(command_list);
            self.pipeline.record_commands(command_list);

            self.vertex_resource.render(command_list);
        }

        self.display.end_frame(command_list, frame_index);
        self.commands.end_frame(device);
        self.display.present();
    }

    pub fn load_image(&mut self, device: &ID3D12Device, img: &Image) {
        self.vertex_resource.load_image(device, img);
    }
}

struct Commands {
    command_allocator: ID3D12CommandAllocator,
    command_queue: ID3D12CommandQueue,
    pub command_list: ID3D12GraphicsCommandList,
    fence: Fence,
}

struct Display {
    swap_chain: IDXGISwapChain4,
    back_buffers: Vec<ID3D12Resource>,
    rtv_heap: ID3D12DescriptorHeap,
}

impl Display {
    const BACK_BUFFER_COUNT: u32 = 2;
    const CLEAR_COLOR: [f32; 4] = [0.0, 0.2, 0.4, 1.0];
}

struct Fence {
    fence: ID3D12Fence,
    value: u64,
    event: HANDLE,
}

impl Commands {
    pub fn new(device: &ID3D12Device) -> Self {
        let command_allocator: ID3D12CommandAllocator =
            match unsafe { device.CreateCommandAllocator(D3D12_COMMAND_LIST_TYPE_DIRECT) } {
                Ok(allocator) => allocator,
                Err(hr) => panic!("Failed to create command allocator: {:?}", hr),
            };

        let command_list: ID3D12GraphicsCommandList = match unsafe {
            device.CreateCommandList(0, D3D12_COMMAND_LIST_TYPE_DIRECT, &command_allocator, None)
        } {
            Ok(list) => list,
            Err(hr) => panic!("Failed to create command list: {:?}", hr),
        };

        let queue_desc = D3D12_COMMAND_QUEUE_DESC {
            Type: D3D12_COMMAND_LIST_TYPE_DIRECT,
            Priority: D3D12_COMMAND_QUEUE_PRIORITY_NORMAL.0,
            Flags: Default::default(),
            NodeMask: 0,
        };
        let command_queue: ID3D12CommandQueue =
            match unsafe { device.CreateCommandQueue(&queue_desc) } {
                Ok(queue) => queue,
                Err(hr) => panic!("Failed to create command queue: {:?}", hr),
            };

        let fence = Fence::new(device);

        Self {
            command_allocator,
            command_queue,
            command_list,
            fence,
        }
    }

    pub fn end_frame(&mut self, device: &ID3D12Device) {
        match unsafe { self.command_list.Close() } {
            Ok(_) => (),
            Err(hr) => {
                match unsafe { device.GetDeviceRemovedReason() } {
                    Ok(reason) => panic!(
                        "Failed to close command list: {:?}, device removed reason: {:?}",
                        hr, reason
                    ),
                    Err(e) => panic!("Failed to close command list: {:?}, and also failed to get device removed reason: {:?}", hr, e),
                }
            }
        }

        let command_lists = [Some(self.command_list.clone().into())];
        unsafe { self.command_queue.ExecuteCommandLists(&command_lists) };

        self.fence.wait(&self.command_queue);

        match unsafe { self.command_allocator.Reset() } {
            Ok(_) => (),
            Err(hr) => panic!("Failed to reset command allocator: {:?}", hr),
        }
        match unsafe { self.command_list.Reset(&self.command_allocator, None) } {
            Ok(_) => (),
            Err(hr) => panic!("Failed to reset command list: {:?}", hr),
        }
    }
}

impl Display {
    pub fn new(
        factory: &IDXGIFactory7,
        device: &ID3D12Device,
        hwnd: &HWND,
        command_queue: &ID3D12CommandQueue,
    ) -> Self {
        let mut rect = RECT::default();
        unsafe {
            GetClientRect(*hwnd, &mut rect);
        }

        let swap_chain_desc = DXGI_SWAP_CHAIN_DESC1 {
            Width: (rect.right - rect.left) as u32,
            Height: (rect.bottom - rect.top) as u32,
            Format: DXGI_FORMAT_R8G8B8A8_UNORM,
            Stereo: false.into(),
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
            BufferCount: Self::BACK_BUFFER_COUNT,
            Scaling: DXGI_SCALING_STRETCH,
            SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
            AlphaMode: DXGI_ALPHA_MODE_UNSPECIFIED,
            Flags: Default::default(),
        };

        let swap_chain: IDXGISwapChain4 = match unsafe {
            factory.CreateSwapChainForHwnd(command_queue, *hwnd, &swap_chain_desc, None, None)
        } {
            Ok(swap_chain) => swap_chain,
            Err(hr) => panic!("Failed to create swap chain: {:?}", hr),
        }
        .cast()
        .expect("Failed to cast swap chain to IDXGISwapChain4");

        let rtv_heap_desc = D3D12_DESCRIPTOR_HEAP_DESC {
            Type: D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
            NumDescriptors: Self::BACK_BUFFER_COUNT,
            Flags: Default::default(),
            NodeMask: 0,
        };
        let rtv_heap: ID3D12DescriptorHeap =
            match unsafe { device.CreateDescriptorHeap(&rtv_heap_desc) } {
                Ok(heap) => heap,
                Err(hr) => panic!("Failed to create RTV descriptor heap: {:?}", hr),
            };

        let got_swap_chain_desc = match unsafe { swap_chain.GetDesc1() } {
            Ok(desc) => desc,
            Err(hr) => panic!("Failed to get swap chain description: {:?}", hr),
        };

        let back_buffers: Vec<ID3D12Resource> = (0..got_swap_chain_desc.BufferCount)
            .map(|i| match unsafe { swap_chain.GetBuffer(i) } {
                Ok(buffer) => buffer,
                Err(hr) => panic!("Failed to get back buffer {}: {:?}", i, hr),
            })
            .collect();

        back_buffers.iter().enumerate().for_each(|(i, buf)| {
            let mut rtv_handle = unsafe { rtv_heap.GetCPUDescriptorHandleForHeapStart() };
            let offset = unsafe {
                device.GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_RTV) as usize * i
            };
            rtv_handle.ptr = rtv_handle.ptr.wrapping_add(offset);

            unsafe { device.CreateRenderTargetView(buf, None, rtv_handle) }
        });

        Self {
            swap_chain,
            back_buffers,
            rtv_heap,
        }
    }

    pub fn begin_frame(
        &self,
        device: &ID3D12Device,
        command_list: &ID3D12GraphicsCommandList,
        frame_index: u32,
    ) {
        let back_buffer = &self.back_buffers[frame_index as usize];

        let barrier = D3D12_RESOURCE_BARRIER {
            Type: D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
            Flags: D3D12_RESOURCE_BARRIER_FLAG_NONE,
            Anonymous: D3D12_RESOURCE_BARRIER_0 {
                Transition: std::mem::ManuallyDrop::new(D3D12_RESOURCE_TRANSITION_BARRIER {
                    pResource: unsafe { std::mem::transmute_copy(back_buffer) },
                    Subresource: 0,
                    StateBefore: D3D12_RESOURCE_STATE_PRESENT,
                    StateAfter: D3D12_RESOURCE_STATE_RENDER_TARGET,
                }),
            },
        };
        unsafe { command_list.ResourceBarrier(&[barrier]) };

        let mut rtv_handle = unsafe { self.rtv_heap.GetCPUDescriptorHandleForHeapStart() };
        let offset = unsafe {
            device.GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_RTV) as usize
                * frame_index as usize
        };
        rtv_handle.ptr = rtv_handle.ptr.wrapping_add(offset);

        unsafe {
            command_list.OMSetRenderTargets(1, Some(&rtv_handle), false, None);
            command_list.ClearRenderTargetView(rtv_handle, &Self::CLEAR_COLOR, None);
        }
    }

    pub fn end_frame(&self, command_list: &ID3D12GraphicsCommandList, frame_index: u32) {
        let back_buffer = &self.back_buffers[frame_index as usize];

        let barrier = D3D12_RESOURCE_BARRIER {
            Type: D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
            Flags: D3D12_RESOURCE_BARRIER_FLAG_NONE,
            Anonymous: D3D12_RESOURCE_BARRIER_0 {
                Transition: std::mem::ManuallyDrop::new(D3D12_RESOURCE_TRANSITION_BARRIER {
                    pResource: unsafe { std::mem::transmute_copy(back_buffer) },
                    Subresource: 0,
                    StateBefore: D3D12_RESOURCE_STATE_RENDER_TARGET,
                    StateAfter: D3D12_RESOURCE_STATE_PRESENT,
                }),
            },
        };
        unsafe { command_list.ResourceBarrier(&[barrier]) };
    }

    pub fn present(&self) {
        let hr = unsafe { self.swap_chain.Present(1, Default::default()) };
        if hr.is_err() {
            panic!("Failed to present swap chain: {:?}", hr);
        }
    }

    pub fn frame_index(&self) -> u32 {
        unsafe { self.swap_chain.GetCurrentBackBufferIndex() }
    }
}

impl Fence {
    pub fn new(device: &ID3D12Device) -> Self {
        let fence: ID3D12Fence = match unsafe { device.CreateFence(0, Default::default()) } {
            Ok(fence) => fence,
            Err(hr) => panic!("Failed to create fence: {:?}", hr),
        };

        let event = unsafe { CreateEventW(None, false, false, None) }
            .expect("Failed to create event for fence synchronization");

        Self {
            fence,
            value: 1,
            event,
        }
    }

    pub fn wait(&mut self, command_queue: &ID3D12CommandQueue) {
        let current_value = self.value;
        match unsafe { command_queue.Signal(&self.fence, current_value) } {
            Ok(_) => (),
            Err(hr) => panic!(
                "Failed to signal command queue for fence synchronization: {:?}",
                hr
            ),
        }

        if let Err(hr) = unsafe { self.fence.SetEventOnCompletion(current_value, self.event) } {
            panic!(
                "Failed to set event on fence completion for synchronization: {:?}",
                hr
            );
        }

        unsafe {
            WaitForSingleObject(self.event, INFINITE);
        }

        self.value += 1;
    }
}
