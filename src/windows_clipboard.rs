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

use std::error::Error;
use std::fmt;

use clipboard_win::{get_clipboard_string, set_clipboard_string, SystemError};

use crate::common::{ClipboardProvider, Result};

// Wrap `clipboard_win::SystemError` since it does not implement `Error` trait
struct WindowsSystemError(SystemError);

impl fmt::Debug for WindowsSystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for WindowsSystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for WindowsSystemError {}

pub struct WindowsClipboardContext;

impl WindowsClipboardContext {
    pub fn new() -> Result<Self> {
        Ok(WindowsClipboardContext)
    }
}

impl ClipboardProvider for WindowsClipboardContext {
    fn get_contents(&mut self) -> Result<String> {
        Ok(get_clipboard_string().map_err(WindowsSystemError)?)
    }

    fn set_contents(&mut self, data: String) -> Result<()> {
        Ok(set_clipboard_string(&data).map_err(WindowsSystemError)?)
    }
}
