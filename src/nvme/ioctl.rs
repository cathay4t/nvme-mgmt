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

use std::fs::File;
use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;

use super::error::*;

#[repr(C, packed)]
#[derive(Default)]
// Copy from /usr/include/linux/nvme_ioctl.h
pub struct NvmeAdminCmd {
    pub opcode:         u8,
    pub flags:          u8,
    pub rsvd1:          u16,
    pub nsid:           u32,
    pub cdw2:           u32,
    pub cdw3:           u32,
    pub metadata:       u64,
    pub addr:           u64,
    pub metadata_len:   u32,
    pub data_len:       u32,
    pub cdw10:          u32,
    pub cdw11:          u32,
    pub cdw12:          u32,
    pub cdw13:          u32,
    pub cdw14:          u32,
    pub cdw15:          u32,
    pub timeout_ms:     u32,
    pub result:         u32,
}

const NVME_IOC_MAGIC: u8 = b'N';
const NVME_IOC_ADMIN_CMD: u8 = 0x41;

ioctl!(readwrite_buf _nvme_ioctl_admin_cmd with
       NVME_IOC_MAGIC, NVME_IOC_ADMIN_CMD;
       NvmeAdminCmd);

pub(crate) fn nvme_ioctl_fd_open(blk_path: &str) -> Result<File> {
    Ok(OpenOptions::new().read(true).open(blk_path)?)
}

pub(crate) fn nvme_ioctl_admin_cmd(fd: &File,
                                   nvme_cmd: NvmeAdminCmd) -> Result<()> {
    let fd_raw: i32 = AsRawFd::as_raw_fd(fd);
    unsafe {_nvme_ioctl_admin_cmd(fd_raw, &mut [nvme_cmd])?;}
    Ok(())
}
