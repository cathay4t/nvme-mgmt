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

use ioctl::{nvme_ioctl_admin_cmd, nvme_ioctl_fd_open, NvmeAdminCmd};
use std::path::Path;
use std::fs::read_dir;
use std::mem::size_of;
use std::str;
use byteorder::{ByteOrder, LittleEndian};

use super::error::*;
use super::spec::{NvmeSpecIdCtrlData};
use super::namespace::NvmeNameSpace;

static SYSFS_NVME_CTRL_FOLDER: &'static str = "/sys/class/nvme/";

const NVME_IOC_CMD_IDENTIFY: u8 = 0x06;
const NVME_ADMIN_CMD_CNS_ALL_CTRL: u32 = 0x01;

pub struct NvmeController {
    raw_id_data:                NvmeSpecIdCtrlData,
    // Converting u8 array to utf8 might have error which we don't want to
    // trigger during getter function, hence we creat them at struct
    // initialize pharse.
    sn:                         String,
    mn:                         String,
    fr:                         String,
    fguid:                      String,
    subnqn:                     String,
    ver_str:                    String,
    blk_path:                   String,
}

fn to_u16(i: [u8; 2]) -> u16 {
    LittleEndian::read_u16(&i)
}

fn to_u32_a3(i: [u8; 3]) -> u32 {
    LittleEndian::read_u24(&i)
}

fn to_u32(i: [u8; 4]) -> u32 {
    LittleEndian::read_u32(&i)
}

impl NvmeController {
    pub fn blk_path_get(&self)  -> &str { &self.blk_path }
    pub fn vid_get(&self)       -> u16  { to_u16(self.raw_id_data.vid) }
    pub fn ssvid_get(&self)     -> u16  { to_u16(self.raw_id_data.ssvid) }
    pub fn sn_get(&self)        -> &str { &self.sn }
    pub fn mn_get(&self)        -> &str { &self.mn }
    pub fn fr_get(&self)        -> &str { &self.fr }
    pub fn rab_get(&self)       -> u8   { self.raw_id_data.rab }
    pub fn ieee_get(&self)      -> u32 { to_u32_a3(self.raw_id_data.ieee) }
    pub fn cmic_get(&self)      -> u8 { self.raw_id_data.cmic }
    pub fn mdts_get(&self)      -> u8 { self.raw_id_data.mdts }
    pub fn cntlid_get(&self)    -> u16 { to_u16(self.raw_id_data.cntlid) }
    pub fn ver_get(&self)       -> u32 { to_u32(self.raw_id_data.ver) }
    pub fn rtd3r_get(&self)     -> u32 { to_u32(self.raw_id_data.rtd3e) }
    pub fn rtd3e_get(&self)     -> u32 { to_u32(self.raw_id_data.rtd3r) }
    pub fn oaes_get(&self)      -> u32 { to_u32(self.raw_id_data.oaes) }
    pub fn ctratt_get(&self)    -> u32 { to_u32(self.raw_id_data.ctratt) }
    pub fn fguid_get(&self)     -> &str { & self.fguid }
    pub fn oacs_get(&self)      -> u16 { to_u16(self.raw_id_data.oacs) }
    pub fn acl_get(&self)       -> u8 { self.raw_id_data.acl }
    pub fn aerl_get(&self)      -> u8 { self.raw_id_data.aerl }
    pub fn frmw_get(&self)      -> u8 { self.raw_id_data.frmw }
    pub fn lpa_get(&self)       -> u8 { self.raw_id_data.lpa }
    pub fn elpe_get(&self)      -> u8 { self.raw_id_data.elpe }
    pub fn npss_get(&self)      -> u8 { self.raw_id_data.npss }
    pub fn avscc_get(&self)     -> u8 { self.raw_id_data.avscc }
    pub fn apsta_get(&self)     -> u8 { self.raw_id_data.apsta }
    pub fn wctemp_get(&self)    -> u16 { to_u16(self.raw_id_data.wctemp) }
    pub fn cctemp_get(&self)    -> u16 { to_u16(self.raw_id_data.cctemp) }
    pub fn mtfa_get(&self)      -> u16 { to_u16(self.raw_id_data.mtfa) }
    pub fn hmpre_get(&self)     -> u32 { to_u32(self.raw_id_data.hmpre) }
    pub fn hmmin_get(&self)     -> u32 { to_u32(self.raw_id_data.hmmin) }
    pub fn tnvmcap_get(&self)   -> &[u8; 16] { &self.raw_id_data.tnvmcap }
    pub fn unvmcap_get(&self)   -> &[u8; 16] { &self.raw_id_data.unvmcap }
    pub fn rpmbs_get(&self)     -> u32 { to_u32(self.raw_id_data.rpmbs) }
    pub fn edstt_get(&self)     -> u16 { to_u16(self.raw_id_data.edstt) }
    pub fn esto_get(&self)      -> u8 { self.raw_id_data.esto }
    pub fn fwug_get(&self)      -> u8 { self.raw_id_data.fwug }
    pub fn kas_get(&self)       -> u16 { to_u16(self.raw_id_data.kas) }
    pub fn hctma_get(&self)     -> u16 { to_u16(self.raw_id_data.hctma) }
    pub fn mntmt_get(&self)     -> u16 { to_u16(self.raw_id_data.mntmt) }
    pub fn mxtmt_get(&self)     -> u16 { to_u16(self.raw_id_data.mxtmt) }
    pub fn sanicap_get(&self)   -> u32 { to_u32(self.raw_id_data.sanicap) }
    pub fn sqes_get(&self)      -> u8 { self.raw_id_data.sqes }
    pub fn cqes_get(&self)      -> u8 { self.raw_id_data.cqes }
    pub fn maxcmd_get(&self)    -> u16 { to_u16(self.raw_id_data.maxcmd) }
    pub fn nn_get(&self)        -> u32 { to_u32(self.raw_id_data.nn) }
    pub fn oncs_get(&self)      -> u16 { to_u16(self.raw_id_data.oncs) }
    pub fn fuses_get(&self)     -> u16 { to_u16(self.raw_id_data.fuses) }
    pub fn fna_get(&self)       -> u8 { self.raw_id_data.fna }
    pub fn vwc_get(&self)       -> u8 { self.raw_id_data.vwc }
    pub fn awun_get(&self)      -> u16 { to_u16(self.raw_id_data.awun) }
    pub fn awupf_get(&self)     -> u16 { to_u16(self.raw_id_data.awupf) }
    pub fn nvscc_get(&self)     -> u8 { self.raw_id_data.nvscc }
    pub fn acwu_get(&self)      -> u16 { to_u16(self.raw_id_data.acwu) }
    pub fn sgls_get(&self)      -> u32 { to_u32(self.raw_id_data.sgls) }
    pub fn subnqn_get(&self)    -> &str { &self.subnqn }
    pub fn ioccsz_get(&self)    -> u32 { to_u32(self.raw_id_data.ioccsz) }
    pub fn iorcsz_get(&self)    -> u32 { to_u32(self.raw_id_data.iorcsz) }
    pub fn icdoff_get(&self)    -> u16 { to_u16(self.raw_id_data.icdoff) }
    pub fn ctrattr_get(&self)   -> u8 { self.raw_id_data.ctrattr }
    pub fn msdbd_get(&self)     -> u8 { self.raw_id_data.msdbd }

    pub fn ver_gen(major: u16, minor: u8, tertiary: u8) -> u32 {
        (major << 16 + minor << 8 + tertiary) as u32
    }

    pub fn ver_str_get(&self)   -> &str { &self.ver_str }

    pub fn namespaces_get(&self) -> Result<Vec<NvmeNameSpace>> {
        let mut ret = Vec::new();
        let blk_path = self.blk_path_get();
        let ns_ids = NvmeNameSpace::ns_id_list_get(blk_path, self)?;
        for ns_id in ns_ids {
            ret.push(NvmeNameSpace::new(blk_path, self, ns_id)?);
        }
        Ok(ret)
    }

    pub fn from_path(blk_path: &str) -> Result<NvmeController> {
        let path =  Path::new(blk_path);

        let fd = nvme_ioctl_fd_open(blk_path)?;

        let mut id_data: NvmeSpecIdCtrlData = Default::default();

        let nvme_cmd = NvmeAdminCmd {
            opcode:             NVME_IOC_CMD_IDENTIFY,
            addr:               &mut id_data as *mut NvmeSpecIdCtrlData as u64,
            data_len:           size_of::<NvmeSpecIdCtrlData>() as u32,
            cdw10:              NVME_ADMIN_CMD_CNS_ALL_CTRL,
            ..                  Default::default()
        };

        nvme_ioctl_admin_cmd(&fd, nvme_cmd)?;

        let mut ver_str = "1.0.0".to_string();
        if to_u32(id_data.ver) != 0 {
            let major = [id_data.ver[2], id_data.ver[3]];
            ver_str = format!("{}.{}.{}", to_u16(major), id_data.ver[1],
                              id_data.ver[0]);
        }

        Ok(NvmeController {
            sn:             str::from_utf8(&id_data.sn)?.trim().to_string(),
            mn:             str::from_utf8(&id_data.mn)?.trim().to_string(),
            fr:             str::from_utf8(&id_data.fr)?.trim().to_string(),
            fguid:          str::from_utf8(&id_data.fguid)?.trim().to_string(),
            subnqn:         str::from_utf8(&id_data.subnqn)?.trim().to_string(),
            raw_id_data:    id_data,
            ver_str:        ver_str,
            blk_path:       format!("{}", blk_path)})
    }

    pub fn get_all() -> Result<Vec<NvmeController>> {
        let mut ret = Vec::new();
        let mut ctrl_names = Vec::new();
        match read_dir(SYSFS_NVME_CTRL_FOLDER) {
            Err(_) => return Ok(ret),
            Ok(paths) => for path in  paths {
                match path {
                    /* Got error when interate, it might happen when
                     * nvme controler got removed after we open that dir
                     */
                    Err(_) => continue,
                    Ok(dir_entry) => ctrl_names.push(dir_entry.file_name()
                                                     .into_string().unwrap())
                };
            }
        };
        for ctrl_name in ctrl_names {
            let ref blk_path = format!("/dev/{}", ctrl_name);
            /* Skip if /dev/nvmeX does not exists */
            if !Path::new(blk_path).exists() {
                continue;
            }
            ret.push(NvmeController::from_path(blk_path)?);
        }
        Ok(ret)
    }
}

