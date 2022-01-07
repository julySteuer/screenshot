#![feature(core_intrinsics)]
use std::ptr::null_mut;
use winapi::shared::windef::{HWND, HGDIOBJ};
use winapi::um::winuser::GetDC;
use winapi::ctypes::{c_void, c_int};
use winapi::shared::minwindef::{DWORD, WORD};
use winapi::shared::ntdef::LONG;
use std::intrinsics::{size_of, offset};
use winapi::um::wingdi::{ GetDeviceCaps, DeleteObject,DeleteDC,HORZRES, VERTRES, GetDIBits,CreateCompatibleDC,CreateCompatibleBitmap, BitBlt, SRCCOPY, SelectObject,RGBQUAD , BITMAPINFO, CAPTUREBLT, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS};
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};
use winapi::um::winuser::{ReleaseDC, GetDesktopWindow};

struct Pixel {
    r: u8,
    g: u8,
    b: u8
}

impl Pixel {
    fn to_im_arr(&self) -> [u8; 3]{
        [self.r, self.g, self.b]
    }
}

fn take_screenshot() -> Vec<u8>{  
    let display = unsafe{GetDesktopWindow()};
    let dc = unsafe{GetDC(display)};
    let width = unsafe{GetDeviceCaps(dc, HORZRES)};
    let height = unsafe{GetDeviceCaps(dc, VERTRES)};
    unsafe {
        let pixel_width: usize = 4;
        let size = (width*height) as usize * pixel_width;
        let mut pixels: Vec<u8> = Vec::with_capacity(size);
        pixels.set_len(size);
        let h_dc = CreateCompatibleDC(dc);
        let bitmap = CreateCompatibleBitmap(dc, width, height);
        SelectObject(dc, bitmap as *mut c_void);
        BitBlt(h_dc, 0, 0, width, height, dc, 0,0,SRCCOPY|CAPTUREBLT);
        let mut bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: size_of::<BITMAPINFOHEADER>() as DWORD,
                biWidth: width as LONG,
                biHeight: height as LONG,
                biPlanes: 1,
                biBitCount: 8*pixel_width as WORD,
                biCompression: BI_RGB,
                biSizeImage: (width * height * pixel_width as c_int) as DWORD,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            },
            bmiColors: [RGBQUAD {
                rgbBlue: 0,
                rgbGreen: 0,
                rgbRed: 0,
                rgbReserved: 0
            }],
        };
        GetDIBits(dc, bitmap, 0, height as DWORD, &mut pixels[0] as *mut u8 as *mut c_void, &mut bmi as *mut BITMAPINFO, DIB_RGB_COLORS);
        ReleaseDC(null_mut(), dc);
        DeleteDC(h_dc);
        DeleteObject(bitmap as *mut c_void);
        let image:RgbImage = ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
            let idx = (y*height as u32 + x*width as u32) as isize;
            let pix = &pixels[0] as *const u8;
            let pixel = Pixel {
                r: *offset(pix, idx+2),
                g: *offset(pix, idx+1),
                b: *offset(pix, idx)
            };
            image::Rgb(pixel.to_im_arr())
        });
        image.save("test.png").unwrap();
        pixels
    }
}

fn main() {
    take_screenshot();
}

