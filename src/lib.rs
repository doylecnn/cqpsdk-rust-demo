#![feature(const_fn)]
#![feature(libc)]

use std::ffi::{CStr,CString};

extern crate libc;
use self::libc::{c_char};

extern crate cqpsdk;
use cqpsdk::{CqpApi,LogLevel};

// static mut cqpapi :CqpApi=cqpsdk::empty_stuct();
static mut cqpapi :CqpApi=CqpApi::static_new();

// https://github.com/rust-lang/rust/issues/17806

#[export_name="\x01_AppInfo"]
pub extern "stdcall" fn app_info() -> *const c_char {
    CString::new("9,me.robirt.rust.demo").unwrap().as_ptr()
}

#[export_name="\x01_Initialize"]
pub extern "stdcall" fn initialize(auth_code: i32) -> i32 {
    unsafe {
        cqpapi = CqpApi::new(auth_code);
    }
    0
}

#[export_name="\x01_PrivateMessageHandler"]
pub extern "stdcall" fn private_message_handler(sub_type: i32, send_time: i32, qq_number: i64, msg: *const c_char, font: i32) -> i32 {
    unsafe {
        cqpapi.send_private_message(qq_number, "Reply!");
        cqpapi.add_log(LogLevel::Info, "demo", "test");
    }
    0
}