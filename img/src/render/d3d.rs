mod device;
mod object;
mod pipeline;
mod resource;

use crate::img::Image;
use crate::render::d3d::device::Device;
use crate::render::d3d::resource::Resources;
use crate::render::renderer::Renderer;
use windows::Win32::Foundation::HWND;

pub struct D3DRenderer {
    device: Device,
    resources: Resources,
}

impl Renderer for D3DRenderer {
    fn new(hwnd: &HWND, img: &Image) -> Self
    where
        Self: Sized,
    {
        let device = Device::new();
        let mut resources = Resources::new(&device.device, &device.dxgi_factory, hwnd);

        resources.load_image(&device.device, img);

        Self { device, resources }
    }

    fn render(&mut self) {
        self.resources.render(&self.device.device);
    }
}
