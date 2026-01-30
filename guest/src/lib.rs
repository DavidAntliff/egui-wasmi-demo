#![no_std]

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

// Panic handler required for no_std
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
