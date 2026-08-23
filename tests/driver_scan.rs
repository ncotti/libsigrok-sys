// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026  Nicolas Gabriel Cotti

use std::ptr::null_mut;

use libsigrok_sys::sigrok::sr_context;
use libsigrok_sys::sigrok::{self as sr, sr_error_code_SR_OK};

#[test]
fn scan_drivers() {
    let mut context: *mut sr_context = null_mut();
    unsafe {
        let status = sr::sr_init(&mut context);
        if status != sr_error_code_SR_OK {
            panic!("sr_init");
        }
    }

    unsafe {
        let mut _drivers = sr::sr_driver_list(context);
    }

    unsafe { sr::sr_exit(context) };
}
