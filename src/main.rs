#[link(name = "clib")]
extern "C" {
    fn square(x: libc::c_int) -> libc::c_int;

    fn span_create(ptr: *const libc::c_void, len: libc::size_t) -> Span;
}

#[repr(C)]
#[derive(Debug)]
struct Span {
    ptr: *const libc::c_void,
    len: libc::size_t,
}

#[repr(C)]
struct SpanPair {
    fst: Span,
    snd: Span,
}

fn safe_square(a: i32) -> i32 {
    unsafe { square(a as libc::c_int) }
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

fn main() {
    let a = 15;

    println!("{}", unsafe { square(2) });
    println!("{}", safe_square(2));
    let span = unsafe {
        span_create(
            &a as *const i32 as *const libc::c_void,
            square(2) as libc::size_t,
        )
    };

    let squared = deep_backtrace(8);
    println!("{squared}");

    let s = String::from("a complex Rust string");
    let hamster = "a less complex Rust string";

    dbg!(span, s, hamster);
}
