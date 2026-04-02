mod device;
mod pipeline;
mod resource;

use crate::render::d3d::device::Device;
use crate::render::d3d::resource::Resources;
use windows::Win32::Foundation::HWND;
use crate::render::renderer::Renderer;

pub struct D3DRenderer {
    device: Device,
    resources: Resources,
}

impl Renderer for D3DRenderer {
    fn new(hwnd: &HWND) -> Self
    where
        Self: Sized,
    {
        let device = Device::new();
        let resources = Resources::new(&device.device, &device.dxgi_factory, hwnd);

        Self { device, resources }
    }

    fn render(&mut self) {
        self.resources.render(&self.device.device);
    }
}
