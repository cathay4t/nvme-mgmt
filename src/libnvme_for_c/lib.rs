/*
 * Copyright (C) 2017 Red Hat, Inc.
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; If not, see <http://www.gnu.org/licenses/>.
 *
 * Author: Gris Ge <fge@redhat.com>
 */

extern crate nvme;
extern crate libc;

use libc::{c_char, c_int};
use std::ffi::CStr;
use std::ffi::CString;
use std::str;

const NVME_OK: c_int = 0;
const NVME_LIBBUG: c_int = 1;

pub struct NvmeCtrlC {
    sn:                 *mut c_char,
}

#[no_mangle]
pub extern fn nvme_ctrl_get(dev_path: *const c_char,
                            cnt: *mut *mut NvmeCtrlC,
                            err_msg: *mut *mut c_char) -> c_int {
    let mut rc = NVME_OK;
    assert!(!dev_path.is_null());
    assert!(!cnt.is_null());
    assert!(!err_msg.is_null());

    let dev_path = unsafe {
        CStr::from_ptr(dev_path)
    }.to_str().unwrap();
    let c = match nvme::NvmeController::from_path(dev_path) {
        Ok(c) => c,
        Err(e) => {
            rc = NVME_LIBBUG;
            let err_msg_rust = format!("{}", e);
            unsafe {*err_msg = CString::new(err_msg_rust).unwrap().into_raw();}
            return rc;
        }
    };
    let c = NvmeCtrlC {
        sn: CString::new(c.sn_get()).unwrap().into_raw(),
    };
    unsafe {
        *cnt = Box::into_raw(Box::new(c));
    }
    rc
}

#[no_mangle]
pub extern fn nvme_ctrl_free(cnt: *mut NvmeCtrlC) {
    if cnt.is_null() { return }
    let cnt = unsafe {
        &mut *cnt
    };
    unsafe {
        CString::from_raw(cnt.sn);
        Box::from_raw(cnt);
    }
}

#[no_mangle]
pub extern fn nvme_ctrl_sn_get(cnt: *mut NvmeCtrlC) -> *const c_char {
    let cnt = unsafe {
        assert!(!cnt.is_null());
        &mut *cnt
    };
    cnt.sn
}

#[no_mangle]
pub extern fn nvme_err_msg_free(err_msg: *mut c_char) {
    unsafe {
        libc::free(err_msg as *mut libc::c_void);
    }
}
