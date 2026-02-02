#![no_std]
#![no_main]

//extern crate alloc; // Panic handler required for no_std
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

// External function provided by the host
// unsafe extern "C" {
//     // fn output(num: u64);
//     //
//     // fn set_pixel(x: u32, y: u32, r: u32, g: u32, b: u32);
//     //
//     // fn fill(r: u32, g: u32, b: u32);
//     //
//     // fn update();
// }

// Don't call the entry 'main' as it will get wrapped with C-style (argc, argv) parameters

// #[no_mangle]
// pub extern "C" fn fib(mut count: u64) -> u64 {
//     let mut a: u64 = 0;
//     let mut b: u64 = 1;
//
//     unsafe {
//         output(a);
//         output(b);
//     }
//
//     while count > 0 {
//         let next = a.wrapping_add(b);
//         unsafe {
//             output(next);
//         }
//
//         a = b;
//         b = next;
//
//         // Prevent overflow by resetting when numbers get too large
//         if next > 1_000_000_000_000_000_000 {
//             a = 0;
//             b = 1;
//         }
//
//         count -= 1;
//     }
//
//     b
// }

#[unsafe(no_mangle)]
pub extern "C" fn add(x: i32, y: i32) -> i32 {
    x + y
}

// #[unsafe(no_mangle)]
// pub extern "C" fn alloc_buffer(size: u32) -> *mut u8 {
//     let buf = vec![0u8; size as usize];
//     let ptr = buf.as_ptr() as *mut u8;
//     core::mem::forget(buf);
//     ptr
// }

// use core::alloc::Layout;
//
// #[unsafe(no_mangle)]
// pub extern "C" fn malloc(size: u32, alignment: u32) -> *mut u8 {
//     unsafe {
//         let layout = Layout::from_size_align_unchecked(size as usize, alignment as usize);
//         alloc::alloc::alloc(layout)
//     }
// }
//
// // This isn't something we expect a user to write.  This will be provided by a language specific SDK
// #[unsafe(no_mangle)]
// pub extern "C" fn free(ptr: *mut u8, size: u32, alignment: u32) {
//     unsafe {
//         let layout = Layout::from_size_align_unchecked(size as usize, alignment as usize);
//         alloc::alloc::dealloc(ptr, layout);
//     }
// }

use core::cell::UnsafeCell;

//static mut PIXEL_BUFFER: UnsafeCell<Vec<u8, { 16 * 16 * 3 }>> = UnsafeCell::new(Vec::new());
const PIXEL_ROWS: usize = 16;
const PIXEL_COLS: usize = 16;
const PIXEL_CHANNELS: usize = 3; // RGB
const PIXEL_BUFFER_SIZE: usize = PIXEL_ROWS * PIXEL_COLS * PIXEL_CHANNELS;

// SAFETY: WASM is single-threaded, so this is safe
struct SyncWrapper<T> {
    inner: UnsafeCell<T>,
}

unsafe impl<T> Sync for SyncWrapper<T> {}

static PIXEL_BUFFER: SyncWrapper<[u8; PIXEL_BUFFER_SIZE]> = SyncWrapper {
    inner: UnsafeCell::new([0u8; PIXEL_BUFFER_SIZE]),
};

#[unsafe(no_mangle)]
pub extern "C" fn buffer_ptr() -> *mut u8 {
    // Get a *mut [u8; PIXEL_BUFFER_SIZE]
    PIXEL_BUFFER.inner.get() as *mut u8
    //    unsafe { PIXEL_BUFFER.get().as_mut_ptr() }
}

// #[unsafe(no_mangle)]
// pub extern "C" fn buffer_ptr_mut() -> *mut u8 {
//     unsafe { PIXEL_BUFFER.as_mut_ptr() }
// }

#[unsafe(no_mangle)]
pub extern "C" fn mem_write(offset: u32, value: u32) {
    let ptr = buffer_ptr();
    if offset as usize >= PIXEL_BUFFER_SIZE {
        panic!("Offset out of bounds");
    }
    unsafe {
        ptr.add(offset as usize).write(value as u8);
    }

    // let ptr = offset as *mut u8;
    // unsafe {
    //     *ptr = value as u8;
    // }
}

#[unsafe(no_mangle)]
pub extern "C" fn mem_read(offset: u32) -> u32 {
    let ptr = buffer_ptr();
    if offset as usize >= PIXEL_BUFFER_SIZE {
        panic!("Offset out of bounds");
    }
    unsafe { ptr.add(offset as usize).read() as u32 }

    // let ptr = offset as *mut u8;
    // unsafe {
    //     *ptr = value as u8;
    // }
}

#[unsafe(no_mangle)]
pub extern "C" fn init() {
    let ptr = buffer_ptr();
    unsafe {
        core::ptr::write_bytes(ptr, 0, PIXEL_BUFFER_SIZE);
    }
}

// #[unsafe(no_mangle)]
// pub extern "C" fn update(frame: u64) {
//     let pixel_buffer = buffer_ptr();
//
//     mem_write(0, (frame % 256) as u32);
//     //pixel_buffer
// }

#[unsafe(no_mangle)]
pub extern "C" fn update(frame: u64) {
    let ptr = buffer_ptr();

    for y in 0..PIXEL_ROWS {
        for x in 0..PIXEL_COLS {
            // Diagonal rainbow: hue based on x + y + frame
            let hue = ((x + y) as u64 * 8 + frame * 2) % 256;

            // Simple HSV to RGB (S=1, V=1)
            let (r, g, b) = hsv_to_rgb(hue as u8);

            let offset = (y * PIXEL_COLS + x) * PIXEL_CHANNELS;
            unsafe {
                ptr.add(offset).write(r);
                ptr.add(offset + 1).write(g);
                ptr.add(offset + 2).write(b);
            }
        }
    }
}

// Simple HSV to RGB with S=1, V=1
fn hsv_to_rgb(hue: u8) -> (u8, u8, u8) {
    let h = hue as u16;
    let sector = h / 43; // 0-5 (256/6 ≈ 43)
    let offset = (h % 43) * 6; // Position within sector, scaled to 0-255

    match sector {
        0 => (255, offset as u8, 0),       // Red -> Yellow
        1 => (255 - offset as u8, 255, 0), // Yellow -> Green
        2 => (0, 255, offset as u8),       // Green -> Cyan
        3 => (0, 255 - offset as u8, 255), // Cyan -> Blue
        4 => (offset as u8, 0, 255),       // Blue -> Magenta
        _ => (255, 0, 255 - offset as u8), // Magenta -> Red
    }
}

// #[no_mangle]
// pub extern "C" fn fill_slow(max_x: u32, max_y: u32, r: u32, g: u32, b: u32) {
//     for x in 0..max_x {
//         for y in 0..max_y {
//             //let val = val.wrapping_add((x as u32).wrapping_mul(31)).wrapping_add(y as u32);
//             unsafe {
//                 set_pixel(x, y, r, g, b);
//             }
//         }
//     }
//     unsafe { update() };
// }
//
// #[no_mangle]
// pub extern "C" fn render(_max_x: u32, _max_y: u32, frames: u32) {
//     let mut r = 0_i32;
//     let mut g = 0_i32;
//     let mut b = 0_i32;
//
//     let mut dr = 2;
//     let mut dg = 3;
//     let mut db = 5;
//
//     for _ in 0..frames {
//         r += dr;
//         g += dg;
//         b += db;
//
//         if r > 255 {
//             r = 255;
//             dr = -dr;
//         }
//         if r < 0 {
//             r = 0;
//             dr = -dr;
//         }
//
//         if g > 255 {
//             g = 255;
//             dg = -dg;
//         }
//         if g < 0 {
//             g = 0;
//             dg = -dg;
//         }
//
//         if b > 255 {
//             b = 255;
//             db = -db;
//         }
//         if b < 0 {
//             b = 0;
//             db = -db;
//         }
//
//         // SAFETY: casting as u32 is safe because values are clamped between 0 and 255
//         unsafe {
//             fill(r as u32, g as u32, b as u32);
//             update()
//         };
//     }
// }
