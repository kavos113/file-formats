use crate::render::d3d::debug::{Debug, enable_debug};
use windows::Win32::Graphics::Direct3D::{
    D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_11_1, D3D_FEATURE_LEVEL_12_0, D3D_FEATURE_LEVEL_12_1,
};
use windows::Win32::Graphics::Direct3D12::{
    D3D12CreateDevice, ID3D12Device,
};
use windows::Win32::Graphics::Dxgi::{
    CreateDXGIFactory2, DXGI_ADAPTER_DESC1, DXGI_ADAPTER_FLAG, DXGI_ADAPTER_FLAG_NONE,
    DXGI_ADAPTER_FLAG_SOFTWARE, DXGI_CREATE_FACTORY_DEBUG, DXGI_ERROR_ACCESS_DENIED,
    DXGI_ERROR_NOT_FOUND, DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE, IDXGIAdapter1, IDXGIFactory7,
};

pub struct Device {
    pub dxgi_factory: IDXGIFactory7,
    pub device: ID3D12Device,
    debug: Debug,
}

impl Device {
    pub fn new() -> Self {
        enable_debug();
        let dxgi_factory = create_factory();
        let adapter = get_adapter(&dxgi_factory);
        let device = create_device(&adapter);

        let mut debug = Debug::new();
        debug.setup_callback(&device);

        Self {
            dxgi_factory,
            device,
            debug,
        }
    }
}

fn create_factory() -> IDXGIFactory7 {
    unsafe {
        CreateDXGIFactory2(DXGI_CREATE_FACTORY_DEBUG)
    }.unwrap_or_else(|hr| {
        if hr.code() == DXGI_ERROR_ACCESS_DENIED {
            panic!("Failed to create DXGI Factory: Access Denied. Make sure you have the necessary permissions.");
        } else {
            panic!("Failed to create DXGI Factory: {:?}", hr);
        }
    })
}

fn create_device(adapter: &IDXGIAdapter1) -> ID3D12Device {
    let feature_levels = [
        D3D_FEATURE_LEVEL_12_1,
        D3D_FEATURE_LEVEL_12_0,
        D3D_FEATURE_LEVEL_11_1,
        D3D_FEATURE_LEVEL_11_0,
    ];

    let mut device: Option<ID3D12Device> = None;
    for &level in &feature_levels {
        match unsafe { D3D12CreateDevice(adapter, level, &mut device) } {
            Ok(_) => {
                return device.unwrap();
            }
            Err(hr) => {
                println!(
                    "Failed to create D3D12 device with feature level {:?}: {:?}",
                    level, hr
                );
            }
        }
    }

    panic!("Failed to create D3D12 device with any supported feature level");
}

fn get_adapter(factory: &IDXGIFactory7) -> IDXGIAdapter1 {
    let mut index: u32 = 0;
    loop {
        let adapter: IDXGIAdapter1 = match unsafe {
            factory.EnumAdapterByGpuPreference(index, DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE)
        } {
            Ok(adapter) => adapter,
            Err(hr) => {
                if hr.code() == DXGI_ERROR_NOT_FOUND {
                    break;
                }

                println!("No more adapters found at index {}", index);
                continue;
            }
        };

        let desc = match unsafe { adapter.GetDesc1() } {
            Ok(desc) => desc,
            Err(_) => panic!("Failed to get adapter description"),
        };

        if (DXGI_ADAPTER_FLAG(desc.Flags as _) & DXGI_ADAPTER_FLAG_SOFTWARE)
            != DXGI_ADAPTER_FLAG_NONE
        {
            index += 1;
            continue;
        }

        return adapter;
    }

    let mut adapters: Vec<(IDXGIAdapter1, DXGI_ADAPTER_DESC1)> = (0..)
        .map_while(|i| match unsafe { factory.EnumAdapters1(i) } {
            Ok(adapter) => {
                let desc = match unsafe { adapter.GetDesc1() } {
                    Ok(desc) => desc,
                    Err(_) => panic!("Failed to get adapter description"),
                };
                if (DXGI_ADAPTER_FLAG(desc.Flags as _) & DXGI_ADAPTER_FLAG_SOFTWARE)
                    != DXGI_ADAPTER_FLAG_NONE
                {
                    None
                } else {
                    Some((adapter, desc))
                }
            }

            Err(hr) => {
                if hr.code() == DXGI_ERROR_NOT_FOUND {
                    None
                } else {
                    println!("No more adapters found at index {}", i);
                    None
                }
            }
        })
        .collect();

    adapters.sort_by(|a, b| {
        b.1.DedicatedVideoMemory
            .cmp(&a.1.DedicatedVideoMemory)
            .then_with(|| b.1.DedicatedSystemMemory.cmp(&a.1.DedicatedSystemMemory))
    });

    if adapters.is_empty() {
        panic!("No suitable adapters found");
    }

    adapters[0].0.clone()
}
