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

error_chain! {
    errors {
        InvalidArgument(msg: String) {
            description("Invalid argument")
            display("Invalid argument: '{}'", msg)
        }
        PermissionDenied(msg: String) {
            description("Permission deny")
            display("Permission deny: '{}'", msg)
        }
        LibBug(msg: String) {
            description("Library bug")
            display("Library bug: '{}'", msg)
        }
    }

    // TODO(Gris Ge): Should use From to map everthing into LibBug
    foreign_links {
        FromUtf8Error(::std::string::FromUtf8Error);
        Utf8Error(::std::str::Utf8Error);
//        NixError(::nix::Error);
    }
}

impl From<::std::io::Error> for Error {
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

impl From<::nix::Error> for Error {
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
