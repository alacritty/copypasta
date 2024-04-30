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

use std::panic::RefUnwindSafe;
use std::panic::UnwindSafe;

use objc2::rc::{autoreleasepool, Id};
use objc2::runtime::ProtocolObject;
use objc2::{msg_send_id, ClassType};
use objc2_app_kit::{NSPasteboard, NSPasteboardTypeFileURL, NSPasteboardTypeString};
use objc2_foundation::{NSArray, NSString, NSURL};

use crate::common::*;

pub struct OSXClipboardContext {
    pasteboard: Id<NSPasteboard>,
}

unsafe impl Send for OSXClipboardContext {}
unsafe impl Sync for OSXClipboardContext {}
impl UnwindSafe for OSXClipboardContext {}
impl RefUnwindSafe for OSXClipboardContext {}

impl OSXClipboardContext {
    pub fn new() -> Result<OSXClipboardContext> {
        // Use `msg_send_id!` instead of `NSPasteboard::generalPasteboard()`
        // in the off case that it will return NULL (even though it's
        // documented not to).
        let pasteboard: Option<Id<NSPasteboard>> =
            unsafe { msg_send_id![NSPasteboard::class(), generalPasteboard] };
        let pasteboard = pasteboard.ok_or("NSPasteboard#generalPasteboard returned null")?;
        Ok(OSXClipboardContext { pasteboard })
    }
}

impl ClipboardProvider for OSXClipboardContext {
    fn get_contents(&mut self) -> Result<String> {
        autoreleasepool(|_| {
            let types = unsafe { self.pasteboard.types() }.unwrap();
            let has_file = unsafe { types.containsObject(NSPasteboardTypeFileURL) };
            let has_str = unsafe { types.containsObject(NSPasteboardTypeString) };

            if !has_str {
                return Err("NSPasteboard#types doesn't contain NSPasteboardTypeString".into());
            }

            let text = if has_file {
                let file_url_string =
                    unsafe { self.pasteboard.stringForType(NSPasteboardTypeFileURL) }
                        .ok_or("NSPasteboard#stringForType returned null")?;

                let file_url = unsafe { NSURL::URLWithString(&file_url_string) }
                    .ok_or("NSURL#URLWithString returned null")?;
                unsafe { file_url.path() }.ok_or("NSURL#path returned null")?
            } else {
                unsafe { self.pasteboard.stringForType(NSPasteboardTypeString) }
                    .ok_or("NSPasteboard#stringForType returned null")?
            };

            Ok(text.to_string())
        })
    }

    fn set_contents(&mut self, data: String) -> Result<()> {
        let string_array =
            NSArray::from_vec(vec![ProtocolObject::from_id(NSString::from_str(&data))]);
        unsafe { self.pasteboard.clearContents() };
        let success = unsafe { self.pasteboard.writeObjects(&string_array) };
        if success {
            Ok(())
        } else {
            Err("NSPasteboard#writeObjects: returned false".into())
        }
    }
}
