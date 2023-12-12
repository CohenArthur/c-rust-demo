/* Embedded setup */

#![no_std]
#![feature(start)]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[start]
fn start(_: isize, _: *const *const u8) -> isize {
    main();

    0
}

/* End of embedded setup */

fn square(a: i32) -> i32 {
    a * a
}

fn id0(a: i32) -> i32 {
    square(a)
}

fn id1(a: i32) -> i32 {
    id0(a)
}

fn id2(a: i32) -> i32 {
    id1(a)
}

fn id3(a: i32) -> i32 {
    id2(a)
}

fn id4(a: i32) -> i32 {
    id3(a)
}

fn id5(a: i32) -> i32 {
    id4(a)
}

fn id6(a: i32) -> i32 {
    id5(a)
}

fn id7(a: i32) -> i32 {
    id6(a)
}

fn id8(a: i32) -> i32 {
    id7(a)
}

fn id9(a: i32) -> i32 {
    id8(a)
}

fn deep_backtrace(a: i32) -> i32 {
    id9(a)
}

fn main() {
    let a = deep_backtrace(15);
    let b = a + 14;
    let c = b * a;
}
