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

#[macro_use]
extern crate nix;
#[macro_use]
extern crate error_chain;
extern crate byteorder;
extern crate libc;
extern crate regex;

pub use self::controller::{NvmeController};
pub use self::namespace::NvmeNameSpace;
pub use self::error::{Error, ErrorKind};
pub use self::utils::size_bytes_2_size_human;

mod error;
mod controller;
mod ioctl;
mod namespace;
mod utils;
mod spec;
