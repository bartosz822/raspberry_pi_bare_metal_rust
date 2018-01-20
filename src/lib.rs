#![feature(core_intrinsics, lang_items, asm)]
#![no_std]

extern crate rlibc;

const SYS_TIMER: u32 = 0x3F003004;

fn get_time() -> u32 {
    let pointer = SYS_TIMER as *mut u32;
    unsafe { *pointer } 
}

#[no_mangle]
pub extern fn kernel_main() {
    let mut state = 1;
    unsafe { SetActLEDState(state) };
    let mut old = get_time();
    loop {
        let curr_time: u32 = get_time(); 
        if curr_time - old > 1000000 {
            state = 1 - state;
            unsafe{ SetActLEDState(state) };
            old = curr_time;
        } 
    }   
}

#[no_mangle]
extern "C"{
    fn SetActLEDState(int: u32);
}

#[lang = "eh_personality"] 
#[no_mangle] 
pub extern fn eh_personality() {}

#[lang="panic_fmt"]
extern fn panic_fmt(_: ::core::fmt::Arguments, _: &'static str, _: u32) -> ! {
    loop {}
}

#[export_name = "_ZN4core9panicking5panic17h52e62394b687c35dE"]
pub fn except_add_overflow() -> ! {
 loop {}
}