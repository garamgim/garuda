#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;
mod vga_buffer;

/*
    panic handler는 패닉이 발생했을 때 호출되는 함수이다.
    여기서는 무한 루프를 돌면서 멈추도록 한다. 
*/
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


/*
    Name mangling은 컴파일러가 함수나 변수의 이름을 변경하는 과정이다.
    Rust에서 fn foo() {}는 실제로 _ZN4main3fooE와 같은 이름으로 변경된다.
    이것은 Rust의 모듈 시스템과 네임스페이스 등을 지원하기 위한 것이다.
    그러나 운영체제의 진입점(entry point)인 _start 함수는 이름이 변경되면 안 된다.
*/

/*
    Calling convention(호출 규약)은 함수가 호출되고 반환되는 방식을 정의한다.
    즉 어떤 레지스터/스택을 쓰는지, 반환값은 어디에 두는지 등을 규정한다.
    Rust는 기본적으로 "Rust" 호출 규약을 사용하지만, 운영체제의 진입점은 C 호출 규약을 따라야 하는것이 사실상 표준이다.
    그래서 extern "C"로 지정한다.
*/

static HELLO: &[u8] = b"Hello World!";

// this function is the entry point, since the linker looks for a function
// named `_start` by default
#[unsafe(no_mangle)]    
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}

// To build this binary
// 1. target embedded ARM system which has no underlying operating system
// cargo build --target thumbv7em-none-eabihf
// 2. compile this for the host system (macOS)
// cargo rustc -- -C link-args="-e __start -static -nostartfiles"