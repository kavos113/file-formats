use crate::img::Image;
use crate::render::d3d::D3DRenderer;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::UpdateWindow;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{
    CS_HREDRAW, CS_VREDRAW, CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW,
    IDC_ARROW, IDI_APPLICATION, LoadCursorW, LoadIconW, MSG, PostQuitMessage, RegisterClassExW,
    SW_SHOW, ShowWindow, TranslateMessage, WINDOW_EX_STYLE, WM_DESTROY, WNDCLASSEXW,
    WS_OVERLAPPEDWINDOW,
};
use windows::core::{PCWSTR, w};

pub trait Renderer {
    fn new(hwnd: &HWND, img: &Image) -> Self
    where
        Self: Sized;

    fn render(&mut self);
}

pub struct Window {
    hwnd: HWND,
    renderer: Box<dyn Renderer>,
}

impl Window {
    const CLASS_NAME: PCWSTR = w!("my_window_class");

    pub fn new(x: i32, y: i32, width: i32, height: i32, img: &Image) -> Self {
        unsafe {
            let instance = GetModuleHandleW(None).unwrap();

            let wc = WNDCLASSEXW {
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(Self::wndproc),
                hInstance: instance.into(),
                hIcon: LoadIconW(None, IDI_APPLICATION).unwrap(),
                hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
                lpszClassName: Self::CLASS_NAME,
                ..Default::default()
            };

            let atom = RegisterClassExW(&wc);
            if atom == 0 {
                panic!("Failed to register window class");
            }

            let hwnd = match CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                Self::CLASS_NAME,
                w!("Hello, Windows!"),
                WS_OVERLAPPEDWINDOW,
                x,
                y,
                width,
                height,
                None,
                None,
                Some(instance.into()),
                None,
            ) {
                Ok(hwnd) => hwnd,
                Err(e) => panic!("Failed to create window: {:?}", e),
            };

            Self {
                hwnd,
                renderer: Box::new(D3DRenderer::new(&hwnd, img)),
            }
        }
    }

    pub fn run(&mut self) {
        unsafe {
            _ = ShowWindow(self.hwnd, SW_SHOW);
            _ = UpdateWindow(self.hwnd);

            let mut msg = MSG::default();
            loop {
                let ret = GetMessageW(&mut msg, None, 0, 0);
                if ret.0 == -1 {
                    panic!("Error in message loop");
                } else if ret.0 == 0 {
                    break;
                } else {
                    _ = TranslateMessage(&msg);
                    _ = DispatchMessageW(&msg);
                }

                self.renderer.render();
            }
        }
    }

    extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        match msg {
            WM_DESTROY => {
                unsafe {
                    PostQuitMessage(0);
                }
                LRESULT(0)
            }

            _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
        }
    }
}
