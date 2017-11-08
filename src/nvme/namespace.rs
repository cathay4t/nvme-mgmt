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
use std::mem::size_of;
use std::str;
use regex::Regex;
use std::fs::OpenOptions;
use std::io::prelude::Read;
use std::fs::read_dir;

use super::ioctl::*;
use super::spec::*;
use super::error::*;
use super::utils::*;
use super::NvmeController;

const NVME_ADMIN_CMD_CNS_ALL_ACTIVE_NS: u32 = 0x02;
const NVME_ADMIN_CMD_CNS_IDENTIFY_NS: u32 = 0x00;
const IDT_NS_MAX_COUNT: usize = 1024;
/* maximum 1024 ns id returned per identify command */

pub struct NvmeNameSpace {
    raw_id_data:                NvmeSpecIdNsData,
    blk_path:                   String,
    ctrl_blk_path:              String,
    nsid:                       u32,
    eui64:                      String,
    nguid:                      String,
}

impl NvmeNameSpace {
    pub fn ctrl_blk_path_get(&self) -> &str {
        &self.ctrl_blk_path
    }

    pub fn nsid_get(&self) -> u32 {
        self.nsid
    }

    pub fn eui64_get(&self) -> &str {
        &self.eui64
    }

    pub fn nguid_get(&self) -> &str {
        &self.nguid
    }

    pub fn size_get(&self) -> u64 {
        let lba_slot = bit_field_extract(self.raw_id_data.flbas, 3, 0) as usize;
        if lba_slot >= MAX_LBA_FORMAT_COUNT {
            return 0;
        }
        let lba_size = 2u64.pow(self.raw_id_data.lbafs[lba_slot].lbads as u32);
        to_u64(self.raw_id_data.nsze) * lba_size
    }

    pub fn blk_path_get(&self) -> &str {
        &self.blk_path
    }

    pub(crate) fn new(ctrl_blk_path: &str, _ctrl: &NvmeController,
                      nsid: u32) -> Result<NvmeNameSpace> {
        let fd = nvme_ioctl_fd_open(ctrl_blk_path)?;

        let mut id_data: NvmeSpecIdNsData = Default::default();

        let nvme_cmd = NvmeAdminCmd {
            opcode:             NVME_IOC_CMD_IDENTIFY,
            addr:               &mut id_data as *mut NvmeSpecIdNsData as u64,
            data_len:           size_of::<NvmeSpecIdNsData>() as u32,
            cdw10:              NVME_ADMIN_CMD_CNS_IDENTIFY_NS,
            nsid:               nsid,
            ..                  Default::default()
        };

        nvme_ioctl_admin_cmd(&fd, nvme_cmd)?;

        let blk_path = match get_blk_path(ctrl_blk_path, nsid) {
            Some(b) => b,
            None => String::new(),
        };

        Ok(NvmeNameSpace{
            //BUG(Gris Ge): blk_path here should be blk_path of namspace.
            ctrl_blk_path:      format!("{}", ctrl_blk_path),
            nsid:               nsid,
            blk_path:           blk_path,
            eui64:              to_hex_string(&id_data.eui64),
            nguid:              to_hex_string(&id_data.nguid),
            raw_id_data:        id_data,
        })
    }

    pub(crate) fn nsid_list_get(blk_path: &str,
                                 ctrl: &NvmeController) -> Result<Vec<u32>> {
        let mut ret = Vec::new();
        let nsid_count = ctrl.nn_get();

        if ctrl.ver_get() < NvmeController::ver_gen(1, 1, 0) {
            /* pre SPEC 1.1.0, Namespaces shall be allocated in
             * order (starting with 1) and packed sequentially.
             */
            for nsid in 1..nsid_count + 1 {
                ret.push(nsid);
            }
            return Ok(ret);
        }
        /* Query all active ns list */
        let fd = nvme_ioctl_fd_open(blk_path)?;

        let mut cur_nsid: u32 = 0;

        while (ret.len() as u32) < nsid_count {
            let mut nsid_list_data = [[0u8; 4]; IDT_NS_MAX_COUNT];
            let nvme_cmd = NvmeAdminCmd {
                opcode:             NVME_IOC_CMD_IDENTIFY,
                addr:               &mut nsid_list_data
                                        as *mut [[u8; 4]; IDT_NS_MAX_COUNT]
                                        as u64,
                data_len:           size_of::<[[u8; 4]; IDT_NS_MAX_COUNT]>()
                                        as u32,
                cdw10:              NVME_ADMIN_CMD_CNS_ALL_ACTIVE_NS,
                nsid:              cur_nsid,
                ..                  Default::default()
            };

            nvme_ioctl_admin_cmd(&fd, nvme_cmd)?;
            for nsid_array in nsid_list_data.iter() {
                let nsid = to_u32(nsid_array.clone());
                if nsid == 0 {
                    break;
                } else {
                    ret.push(nsid);
                    cur_nsid = nsid;
                }
            }
        }

        Ok(ret)
    }
}

fn get_nsid_from_sysfs(ctrl_name: &str, ns_name: &str) -> u32 {
    let sysfs_path = format!("/sys/class/nvme/{}/{}/nsid",
                             ctrl_name, ns_name);

    let mut fd = match OpenOptions::new().read(true).open(sysfs_path) {
        Err(_) => return 0u32,
        Ok(i) => i,
    };

    let mut contents = String::new();
    match fd.read_to_string(&mut contents) {
        Err(_) => return 0u32,
        Ok(_) => ()
    }
    /* Remove trailing '\n' */
    contents.trim().trim();
    let re = Regex::new(r"^([0-9]+)\n$").unwrap();
    let nsid = match re.captures(&contents) {
        Some(caps) => format!("{}", &caps[1]),
        None => return 0u32,
    };

    nsid.parse::<u32>().unwrap()
}

fn get_blk_path(ctrl_blk_path: &str, nsid: u32) -> Option<String> {
    let re = Regex::new(r"^/dev/(nvme[0-9]+)$").unwrap();
    let ctrl_name = match re.captures(ctrl_blk_path) {
        Some(caps) => format!("{}", &caps[1]),
        None => return None
    };
    /* In best chance linux kernel just match nsid to /dev/nvme1nX.
     * We try first before doing deep look up.
     */
    let ns_name = &format!("{}n{}", ctrl_name, nsid);
    if get_nsid_from_sysfs(&ctrl_name, &ns_name) == nsid {
        return Some(format!("/dev/{}", ns_name));
    }

    /* Have to do the hard way by search all */
    let sysfs_path = format!("/sys/class/nvme/{}/", ctrl_name);
    let re = Regex::new(r"^(nvme[0-9]+n[0-9]+)$").unwrap();
    match read_dir(&sysfs_path) {
        Err(_) => return None,
        Ok(paths) => for path in paths {
            match path {
                /* Got error when interate, it might happen when
                 * nvme controler got removed after we open that dir
                 */
                Err(_) => continue,
                Ok(dir_entry) => {
                    let ns_name = dir_entry.file_name()
                        .into_string().unwrap();
                    if re.is_match(&ns_name) &&
                       get_nsid_from_sysfs(&ctrl_name, &ns_name) == nsid {
                        return Some(format!("/dev/{}", ns_name));
                    }
                    continue;
                }
            };
        }
    };

    None
}
