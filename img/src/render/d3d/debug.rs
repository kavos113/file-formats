use std::collections::HashMap;
use std::ffi::CStr;
use std::hash::Hash;
use std::sync::OnceLock;
use windows::Win32::Graphics::Direct3D12::{
    D3D12_MESSAGE_CALLBACK_FLAG_NONE, D3D12_MESSAGE_CATEGORY,
    D3D12_MESSAGE_CATEGORY_APPLICATION_DEFINED, D3D12_MESSAGE_CATEGORY_CLEANUP,
    D3D12_MESSAGE_CATEGORY_COMPILATION, D3D12_MESSAGE_CATEGORY_EXECUTION,
    D3D12_MESSAGE_CATEGORY_INITIALIZATION, D3D12_MESSAGE_CATEGORY_MISCELLANEOUS,
    D3D12_MESSAGE_CATEGORY_RESOURCE_MANIPULATION, D3D12_MESSAGE_CATEGORY_SHADER,
    D3D12_MESSAGE_CATEGORY_STATE_CREATION, D3D12_MESSAGE_CATEGORY_STATE_GETTING,
    D3D12_MESSAGE_CATEGORY_STATE_SETTING, D3D12_MESSAGE_ID, D3D12_MESSAGE_SEVERITY,
    D3D12_MESSAGE_SEVERITY_CORRUPTION, D3D12_MESSAGE_SEVERITY_ERROR, D3D12_MESSAGE_SEVERITY_INFO,
    D3D12_MESSAGE_SEVERITY_MESSAGE, D3D12_MESSAGE_SEVERITY_WARNING, D3D12GetDebugInterface,
    ID3D12Debug1, ID3D12Device, ID3D12InfoQueue, ID3D12InfoQueue1,
};
use windows::Win32::Graphics::Dxgi::DXGI_ERROR_ACCESS_DENIED;
use windows::core::{Interface, PCSTR};

pub struct Debug {
    cookies: u32,
}

impl Debug {
    pub fn new() -> Self {
        Self { cookies: 0 }
    }

    fn category_map() -> &'static HashMap<i32, &'static str> {
        static MAP: OnceLock<HashMap<i32, &'static str>> = OnceLock::new();
        MAP.get_or_init(|| {
            HashMap::from([
                (
                    D3D12_MESSAGE_CATEGORY_APPLICATION_DEFINED.0,
                    "Application Defined",
                ),
                (D3D12_MESSAGE_CATEGORY_MISCELLANEOUS.0, "Miscellaneous"),
                (D3D12_MESSAGE_CATEGORY_INITIALIZATION.0, "Initialization"),
                (D3D12_MESSAGE_CATEGORY_CLEANUP.0, "Cleanup"),
                (D3D12_MESSAGE_CATEGORY_COMPILATION.0, "Compilation"),
                (D3D12_MESSAGE_CATEGORY_STATE_CREATION.0, "State Creation"),
                (D3D12_MESSAGE_CATEGORY_STATE_SETTING.0, "State Setting"),
                (D3D12_MESSAGE_CATEGORY_STATE_GETTING.0, "State Getting"),
                (
                    D3D12_MESSAGE_CATEGORY_RESOURCE_MANIPULATION.0,
                    "Resource Manipulation",
                ),
                (D3D12_MESSAGE_CATEGORY_EXECUTION.0, "Execution"),
                (D3D12_MESSAGE_CATEGORY_SHADER.0, "Shader"),
            ])
        })
    }

    fn severity_map() -> &'static HashMap<i32, &'static str> {
        static MAP: OnceLock<HashMap<i32, &'static str>> = OnceLock::new();
        MAP.get_or_init(|| {
            HashMap::from([
                (D3D12_MESSAGE_SEVERITY_CORRUPTION.0, "[CORRUPTION]"),
                (D3D12_MESSAGE_SEVERITY_ERROR.0, "[   ERROR  ]"),
                (D3D12_MESSAGE_SEVERITY_WARNING.0, "[  WARNING ]"),
                (D3D12_MESSAGE_SEVERITY_INFO.0, "[   INFO   ]"),
                (D3D12_MESSAGE_SEVERITY_MESSAGE.0, "[  MESSAGE ]"),
            ])
        })
    }

    unsafe extern "system" fn debug_callback(
        category: D3D12_MESSAGE_CATEGORY,
        severity: D3D12_MESSAGE_SEVERITY,
        message_id: D3D12_MESSAGE_ID,
        description: PCSTR,
        context: *mut std::ffi::c_void,
    ) {
        let category_str = Self::category_map()
            .get(&category.0)
            .unwrap_or(&"Unknown Category");
        let severity_str = Self::severity_map()
            .get(&severity.0)
            .unwrap_or(&"[Unknown Severity]");
        let description_str = CStr::from_ptr(description.as_ptr() as _)
            .to_string_lossy()
            .into_owned();

        println!("{} ({}): {}", severity_str, category_str, description_str);
    }

    pub fn setup_callback(&mut self, device: &ID3D12Device) {
        let info_queue: ID3D12InfoQueue1 = device
            .cast()
            .expect("Failed to get ID3D12InfoQueue1 interface");
        match unsafe {
            info_queue.RegisterMessageCallback(
                Some(Self::debug_callback),
                D3D12_MESSAGE_CALLBACK_FLAG_NONE,
                std::ptr::null_mut(),
                &mut self.cookies,
            )
        } {
            Ok(_) => println!("D3D12 debug callback registered successfully"),
            Err(hr) => println!("Failed to register D3D12 debug callback: {:?}", hr),
        }
    }
}

pub fn enable_debug() {
    let mut debug_controller: Option<ID3D12Debug1> = None;
    match unsafe { D3D12GetDebugInterface(&mut debug_controller) } {
        Ok(_) => (),
        Err(hr) => {
            if hr.code() == DXGI_ERROR_ACCESS_DENIED {
                println!(
                    "Warning: Failed to enable D3D12 debug layer: Access Denied. Make sure you have the necessary permissions."
                );
            } else {
                println!("Warning: Failed to enable D3D12 debug layer: {:?}", hr);
            }
        }
    }

    let debug_controller = match debug_controller {
        Some(controller) => controller,
        None => return,
    };

    unsafe {
        debug_controller.EnableDebugLayer();
        debug_controller.SetEnableGPUBasedValidation(true);
    }

    println!("D3D12 debug layer enabled");
}
