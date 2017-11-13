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
use std::result;
use std::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    LibBug(String),
    InvalidArgument(String),
    PermissionDenied(String),
    CorruptedData(String),
}

#[derive(Debug)]
pub struct NvmeError {
    pub kind:       ErrorKind,
}

pub type Result<T> = result::Result<T, NvmeError>;


impl From<ErrorKind> for NvmeError {
    fn from(ek: ErrorKind) -> NvmeError {
        NvmeError{kind: ek}
    }
}

impl fmt::Display for NvmeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self.kind {
            ErrorKind::LibBug(ref x) => x,
            ErrorKind::InvalidArgument(ref x) => x,
            ErrorKind::PermissionDenied(ref x) => x,
            ErrorKind::CorruptedData(ref x) => x,
        })
    }
}


impl ::std::error::Error for NvmeError {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::LibBug(_) => "Library bug",
            ErrorKind::InvalidArgument(_) => "Invalid argument",
            ErrorKind::PermissionDenied(_) => "Permission denied",
            ErrorKind::CorruptedData(_) =>
                "Corrupted data from NVMe controller",
        }
    }
}


impl From<::std::str::Utf8Error> for NvmeError {
    fn from(e: ::std::str::Utf8Error) -> Self {
        // TODO
        ErrorKind::LibBug(format!("{}", e)).into()
    }
}

impl From<::std::io::Error> for NvmeError {
    fn from(e: ::std::io::Error) -> Self {
        match e.kind() {
            ::std::io::ErrorKind::NotFound =>
                ErrorKind::InvalidArgument(format!("{}", e)).into(),
            ::std::io::ErrorKind::PermissionDenied =>
                ErrorKind::PermissionDenied(format!("{}", e)).into(),
            _ => ErrorKind::LibBug(format!("{}", e)).into()
        }
    }
}

impl From<::nix::Error> for NvmeError {
    fn from(e: ::nix::Error) -> Self {
        match e {
            ::nix::Error::Sys(errno) =>
                ErrorKind::LibBug(format!("ioctl failed: {}", errno)).into(),
            ::nix::Error::InvalidPath =>
                ErrorKind::InvalidArgument(format!("Invalid path: {}", e))
                .into(),
            _ => ErrorKind::LibBug(format!("{}", e)).into()
        }
    }
}
