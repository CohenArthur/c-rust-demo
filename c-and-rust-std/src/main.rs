#![no_std]
#![feature(c_size_t)]
#![feature(start)]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[link(name = "clib")]
extern "C" {
    fn square(x: core::ffi::c_int) -> core::ffi::c_int;

    fn span_create(ptr: *const core::ffi::c_void, len: core::ffi::c_size_t) -> Span;
    fn show_span_pair(ptr: *const SpanPair);
}

#[repr(C)]
#[derive(Debug)]
struct Span {
    ptr: *const core::ffi::c_void,
    len: core::ffi::c_size_t,
}

#[repr(C)]
#[derive(Debug)]
struct SpanPair {
    fst: Span,
    snd: Span,
}

fn safe_square(a: i32) -> i32 {
    unsafe { square(a as core::ffi::c_int) }
}

fn id0(a: i32) -> i32 {
    safe_square(a)
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

#[start]
fn start(_: isize, _: *const *const u8) -> isize {
    main()
}

fn main() -> isize {
    let a = 15;

    // println!("{}", unsafe { square(2) });
    // println!("{}", safe_square(2));

    let span = unsafe {
        span_create(
            &a as *const i32 as *const core::ffi::c_void,
            square(2) as core::ffi::c_size_t,
        )
    };

    let span2 = unsafe {
        span_create(
            &span as *const Span as *const core::ffi::c_void,
            1 as core::ffi::c_size_t,
        )
    };

    let pair = SpanPair {
        fst: span,
        snd: span2,
    };

    // dbg!(&pair);
    unsafe { show_span_pair(&pair as *const SpanPair) };

    let squared = deep_backtrace(8);
    // dbg!(squared);

    let s = "a less complex Rust string";

    squared as isize + s.len() as isize
}
