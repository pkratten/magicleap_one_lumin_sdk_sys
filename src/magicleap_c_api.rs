//Source: https://github.com/servo/webxr
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::ffi::CStr;

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MLResult(u32);

impl MLResult {
    pub const Ok: Self = MLResult(MLResultGlobal_MLResult_Ok);
    pub const Timeout: Self = MLResult(MLResultGlobal_MLResult_Timeout);
    pub const UnspecifiedFailure: Self = MLResult(MLResultGlobal_MLResult_UnspecifiedFailure);

    pub fn ok(self) -> Result<(), MLResult> {
        if self == Self::Ok {
            Ok(())
        } else {
            Err(self)
        }
    }
}

impl From<MLResult> for String {
    fn from(result: MLResult) -> String {
        let cstr = unsafe { CStr::from_ptr(MLSnapshotGetResultString(result)) };
        cstr.to_string_lossy().into_owned()
    }
}

include!(concat!(env!("OUT_DIR"), "/magicleap_c_api.rs"));
