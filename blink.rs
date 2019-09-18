#![no_std]
#![no_main]

const WDTPW: u16 = 0x5A00;
const WDTHOLD: u16 = 0x0080;

extern "C" {
    static P1IN: u8;
    static mut P1OUT: u8;
    static mut P1DIR: u8;
    /* watchdog */
    static mut WDTCTL: u16;
}

#[export_name = "main"]
pub fn start() {
    let mut i = 0;
    unsafe {
        /* watchdog off */
        WDTCTL = WDTPW | WDTHOLD;
        /* configure io */
        P1DIR = 0b01000001;
        P1OUT = 0b00000000;
    }

    loop {
        P1OUT ^= 0b01000000;
        i = 0;
        while i < 5000 {
            i += 1;
        }
    }
}
#[lang = "panic_fmt"]
fn panic_fmt() -> ! {
    loop {}
}
