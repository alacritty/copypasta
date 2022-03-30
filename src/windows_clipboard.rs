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

use crate::common;
use clipboard_win::{get_clipboard_string, set_clipboard_string};
use std::io::{Error, ErrorKind};

pub struct WindowsClipboardContext;

impl WindowsClipboardContext {
    pub fn new() -> common::Result<Self> {
        Ok(WindowsClipboardContext)
    }
}

impl common::ClipboardProvider for WindowsClipboardContext {
    fn get_contents(&mut self) -> common::Result<String> {
        if let Ok(data) = get_clipboard_string() {
            Ok(data)
        } else {
            Err(Box::new(Error::new(ErrorKind::Other, "unable to get clipboard")))
        }
    }

    fn set_contents(&mut self, data: String) -> common::Result<()> {
        if set_clipboard_string(&data).is_ok() {
            Ok(())
        } else {
            Err(Box::new(Error::new(ErrorKind::Other, "unable to set clipboard")))
        }
    }
}
