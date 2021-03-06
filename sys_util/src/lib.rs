// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
//
// Portions Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the THIRD-PARTY file.

//! Small system utility modules for usage by other modules.

extern crate libc;

#[macro_use]
pub mod ioctl;

mod errno;
mod eventfd;
mod signal;
mod struct_util;
mod terminal;

pub use errno::{errno_result, Error, Result};
pub use eventfd::*;
pub use ioctl::*;
pub use signal::*;
pub use struct_util::*;
pub use terminal::*;
