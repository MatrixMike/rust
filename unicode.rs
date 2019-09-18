#![no_main]
#![no_std]
extern crate unicode_xid;

use unicode_xid::UnicodeXID;

fn main() {
    let ch = 'a';
    println!("Is {} a valid start of an identifier? {}", ch, UnicodeXID::is_xid_start(ch));
}
