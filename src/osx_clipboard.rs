// Copyright 2016 Avraham Weinstock
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::common::*;
use objc::runtime::{Class, Object, Sel};
use objc::{msg_send, sel, sel_impl};
use objc_foundation::{INSArray, INSString};
use objc_foundation::{NSArray, NSString};
use objc_id::Id;
use std::ffi::CString;

pub struct OSXClipboardContext {
    pasteboard: Id<Object>,
}

// required to bring NSPasteboard into the path of the class-resolver
#[link(name = "AppKit", kind = "framework")]
extern "C" {
    pub static NSPasteboardTypeString: Sel;
}

impl OSXClipboardContext {
    pub fn new() -> Result<OSXClipboardContext> {
        let cls = Class::get("NSPasteboard").ok_or("Class::get(\"NSPasteboard\")")?;
        let pasteboard: *mut Object = unsafe { msg_send![cls, generalPasteboard] };
        if pasteboard.is_null() {
            return Err("NSPasteboard#generalPasteboard returned null".into());
        }
        let pasteboard: Id<Object> = unsafe { Id::from_ptr(pasteboard) };
        Ok(OSXClipboardContext { pasteboard })
    }
}

impl ClipboardProvider for OSXClipboardContext {
    fn get_contents(&mut self) -> Result<String> {
        let string: *mut NSString =
            unsafe { msg_send![self.pasteboard, stringForType: NSPasteboardTypeString] };
        if string.is_null() {
            Err("pasteboard#stringForType returned null".into())
        } else {
            Ok(nsstring_to_rust_string(unsafe { Id::from_retained_ptr(string) }))
        }
    }

    fn set_contents(&mut self, data: String) -> Result<()> {
        let string_array = NSArray::from_vec(vec![NSString::from_str(&data)]);
        let _: () = unsafe { msg_send![self.pasteboard, clearContents] };
        let success: bool = unsafe { msg_send![self.pasteboard, writeObjects: string_array] };
        if success {
            Ok(())
        } else {
            Err("NSPasteboard#writeObjects: returned false".into())
        }
    }
}

/// Function that converts NSString to rust string through CString to prevent a memory leak.
///
/// encoding:
/// 4 = NSUTF8StringEncoding
/// https://developer.apple.com/documentation/foundation/1497293-string_encodings/nsutf8stringencoding?language=objc
///
/// getCString:
/// Converts the string to a given encoding and stores it in a buffer.
/// https://developer.apple.com/documentation/foundation/nsstring/1415702-getcstring
fn nsstring_to_rust_string(nsstring: Id<NSString>) -> String {
    let string_size: usize = unsafe { msg_send![nsstring, lengthOfBytesUsingEncoding: 4] };
    let mut buffer: Vec<u8> = vec![0_u8; string_size + 1];
    let is_success: bool = unsafe {
        msg_send![nsstring, getCString:buffer.as_mut_ptr()  maxLength:string_size+1 encoding:4]
    };
    if is_success {
        // before from_vec_with_nul can be used https://github.com/rust-lang/rust/pull/89292
        // nul termination from the buffer should be removed by hands
        buffer.pop();

        unsafe { CString::from_vec_unchecked(buffer).to_str().unwrap().to_string() }
    } else {
        // In case getCString failed there is no point in creating CString
        // Original NSString::as_str() swallows all the errors.
        // Not sure if that is the correct approach, but we also don`t have errors here.
        "".to_string()
    }
}
