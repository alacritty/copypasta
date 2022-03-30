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

pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync + 'static>>;

// TODO: come up with some platform-agnostic API for richer types
/// Trait for clipboard access
pub trait ClipboardProvider: Send {
    /// Method to get the clipboard contents as a String
    fn get_contents(&mut self) -> Result<String>;
    /// Method to set the clipboard contents as a String
    fn set_contents(&mut self, data: String) -> Result<()>;
}
