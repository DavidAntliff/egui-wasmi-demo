#![no_std]
#![no_main]

// Panic handler required for no_std
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

use core::cell::UnsafeCell;

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

//static STATIC_0001_IMAGE_DATA: &[u8] = include_bytes!("../assets/static-0001.raw");

static ANIM_0001_IMAGE_DATA: [&[u8; 768]; 6] = [
    include_bytes!("../assets/anim-0001_000.raw"),
    include_bytes!("../assets/anim-0001_001.raw"),
    include_bytes!("../assets/anim-0001_002.raw"),
    include_bytes!("../assets/anim-0001_003.raw"),
    include_bytes!("../assets/anim-0001_004.raw"),
    include_bytes!("../assets/anim-0001_005.raw"),
];

#[unsafe(no_mangle)]
pub extern "C" fn buffer_ptr() -> *mut u8 {
    PIXEL_BUFFER.inner.get() as *mut u8
}

#[unsafe(no_mangle)]
pub extern "C" fn mem_write(offset: u32, value: u32) {
    let ptr = buffer_ptr();
    if offset as usize >= PIXEL_BUFFER_SIZE {
        panic!("Offset out of bounds");
    }
    unsafe {
        ptr.add(offset as usize).write(value as u8);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn mem_read(offset: u32) -> u32 {
    let ptr = buffer_ptr();
    if offset as usize >= PIXEL_BUFFER_SIZE {
        panic!("Offset out of bounds");
    }
    unsafe { ptr.add(offset as usize).read() as u32 }
}

#[unsafe(no_mangle)]
pub extern "C" fn init() {
    let ptr = buffer_ptr();
    unsafe {
        core::ptr::write_bytes(ptr, 0, PIXEL_BUFFER_SIZE);
    }
}

/// Returns the offset to the pixel buffer to be displayed
#[unsafe(no_mangle)]
pub extern "C" fn update(frame: u64, host_buffer_offset: u32) -> u32 {
    match frame % 600 {
        0..200 => rainbow_cycle(frame, host_buffer_offset),
        200..400 => proc0001(frame, host_buffer_offset),
        400..600 => anim0001(frame, host_buffer_offset),
        _ => panic!("Unreachable"),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rainbow_cycle(frame: u64, host_buffer_offset: u32) -> u32 {
    //let ptr = buffer_ptr();
    // Use the host-provided buffer
    let ptr = host_buffer_offset as *mut u8;

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

    host_buffer_offset
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

#[unsafe(no_mangle)]
pub extern "C" fn anim0001(frame: u64, _host_buffer_offset: u32) -> u32 {
    // Use our own (static) buffers
    // Scale down the frame number to control animation speed
    let anim_frame = (frame / 16) % ANIM_0001_IMAGE_DATA.len() as u64;
    let frame_data = ANIM_0001_IMAGE_DATA[anim_frame as usize];
    frame_data.as_ptr() as u32
}

#[unsafe(no_mangle)]
pub extern "C" fn proc0001(frame: u64, _host_buffer_offset: u32) -> u32 {
    // Use our own buffer
    let ptr = buffer_ptr();

    for y in 0..PIXEL_ROWS {
        if y % 2 == 0 {
            continue;
        }
        for x in 0..PIXEL_COLS {
            if x % 2 == 0 {
                continue;
            }
            let hue = (x as u64 + frame) % 256;
            //let hue = ((x + y) as u64 * 8 + frame * 2) % 256;

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

    ptr as u32
}
