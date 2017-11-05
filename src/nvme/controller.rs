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

use ioctl::nvme_ioctl_admin_cmd;
use std::path::Path;
use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;
use std::mem::size_of;
use ioctl::NvmeAdminCmd;
use std::str;
use error::*;
use byteorder::{ByteOrder, LittleEndian};

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub(crate) struct NvmeSpecPsd {
    mp:             [u8; 2],
    reserved_0:     u8,
    _bit_field_0:   u8,        /* mxps:1, nops:1, reserve:6 */
    enlat:          [u8; 4],
    exlat:          [u8; 4],
    _bit_field_1:   u8,        /* rrt:5, reserve:3 */
    _bit_field_2:   u8,        /* rrl:5, reserved:3 */
    _bit_field_3:   u8,        /* rwt:5, reserved:3 */
    _bit_field_4:   u8,        /* rwl:5, reserved:3 */
    idlp:           [u8; 2],
    _bit_field_5:   u8,        /* reserved:6, ips:2 */
    reserved_7:     u8,
    actp:           [u8; 2],
    _bit_field_6:   u8,        /* apw:3, reserved:3, aps:2 */
    reserved_9:     [u8; 9],
}

impl Default for NvmeSpecPsd {
    fn default() -> NvmeSpecPsd {
        NvmeSpecPsd {
            mp:             [0; 2],
            reserved_0:     0,
            _bit_field_0:   0,
            enlat:          [0; 4],
            exlat:          [0; 4],
            _bit_field_1:   0,
            _bit_field_2:   0,
            _bit_field_3:   0,
            _bit_field_4:   0,
            idlp:           [0; 2],
            _bit_field_5:   0,
            reserved_7:     0,
            actp:           [0; 2],
            _bit_field_6:   0,
            reserved_9:     [0; 9],
        }
    }
}

#[repr(C, packed)]
pub(crate) struct NvmeSpecIdCtrlData {
    vid:                [u8; 2],
    ssvid:              [u8; 2],
    sn:                 [u8; 20],
    mn:                 [u8; 40],
    fr:                 [u8; 8],
    rab:                u8,
    ieee:               [u8; 3],
    cmic:               u8,
    mdts:               u8,
    cntlid:             [u8; 2],
    ver:                [u8; 4],
    rtd3r:              [u8; 4],
    rtd3e:              [u8; 4],
    oaes:               [u8; 4],
    ctratt:             [u8; 4],
    reserved_0:         [u8; 12],
    fguid:              [u8; 16],
    reserved_1:         [u8; 112],
    ressered_mi:        [u8; 16],
    oacs:               [u8; 2],
    acl:                u8,
    aerl:               u8,
    frmw:               u8,
    lpa:                u8,
    elpe:               u8,
    npss:               u8,
    avscc:              u8,
    apsta:              u8,
    wctemp:             [u8; 2],
    cctemp:             [u8; 2],
    mtfa:               [u8; 2],
    hmpre:              [u8; 4],
    hmmin:              [u8; 4],
    tnvmcap:            [u8; 16],
    unvmcap:            [u8; 16],
    rpmbs:              [u8; 4],
    edstt:              [u8; 2],
    esto:               u8,
    fwug:               u8,
    kas:                [u8; 2],
    hctma:              [u8; 2],
    mntmt:              [u8; 2],
    mxtmt:              [u8; 2],
    sanicap:            [u8; 4],
    reserved_2:         [u8; 180],
    sqes:               u8,
    cqes:               u8,
    maxcmd:             [u8; 2],
    nn:                 [u8; 4],
    oncs:               [u8; 2],
    fuses:              [u8; 2],
    fna:                u8,
    vwc:                u8,
    awun:               [u8; 2],
    awupf:              [u8; 2],
    nvscc:              u8,
    reserved_3:         u8,
    acwu:               [u8; 2],
    reserved_4:         [u8; 2],
    sgls:               [u8; 4],
    reserved_5:         [u8; 228],
    subnqn:             [u8; 256],
    reserved_6:         [u8; 768],
    /* Below are for NVMe Fabric */
    ioccsz:             [u8; 4],
    iorcsz:             [u8; 4],
    icdoff:             [u8; 2],
    ctrattr:            u8,
    msdbd:              u8,
    reserved_7:         [u8; 244],
    /* Above are for NVMe Fabric */
    psds:               [NvmeSpecPsd; 32],
    vendor_specific:    [u8; 1024],
}

impl Default for NvmeSpecIdCtrlData {
    fn default() -> NvmeSpecIdCtrlData {
        NvmeSpecIdCtrlData {
            vid:                [0; 2],
            ssvid:              [0; 2],
            sn:                 [0; 20],
            mn:                 [0; 40],
            fr:                 [0; 8],
            rab:                0,
            ieee:               [0; 3],
            cmic:               0,
            mdts:               0,
            cntlid:             [0; 2],
            ver:                [0; 4],
            rtd3r:              [0; 4],
            rtd3e:              [0; 4],
            oaes:               [0; 4],
            ctratt:             [0; 4],
            reserved_0:         [0; 12],
            fguid:              [0; 16],
            reserved_1:         [0; 112],
            ressered_mi:        [0; 16],
            oacs:               [0; 2],
            acl:                0,
            aerl:               0,
            frmw:               0,
            lpa:                0,
            elpe:               0,
            npss:               0,
            avscc:              0,
            apsta:              0,
            wctemp:             [0; 2],
            cctemp:             [0; 2],
            mtfa:               [0; 2],
            hmpre:              [0; 4],
            hmmin:              [0; 4],
            tnvmcap:            [0; 16],
            unvmcap:            [0; 16],
            rpmbs:              [0; 4],
            edstt:              [0; 2],
            esto:               0,
            fwug:               0,
            kas:                [0; 2],
            hctma:              [0; 2],
            mntmt:              [0; 2],
            mxtmt:              [0; 2],
            sanicap:            [0; 4],
            reserved_2:         [0; 180],
            sqes:               0,
            cqes:               0,
            maxcmd:             [0; 2],
            nn:                 [0; 4],
            oncs:               [0; 2],
            fuses:              [0; 2],
            fna:                0,
            vwc:                0,
            awun:               [0; 2],
            awupf:              [0; 2],
            nvscc:              0,
            reserved_3:         0,
            acwu:               [0; 2],
            reserved_4:         [0; 2],
            sgls:               [0; 4],
            reserved_5:         [0; 228],
            subnqn:             [0; 256],
            reserved_6:         [0; 768],
            ioccsz:             [0; 4],
            iorcsz:             [0; 4],
            icdoff:             [0; 2],
            ctrattr:            0,
            msdbd:              0,
            reserved_7:         [0; 244],
            psds:               [Default::default(); 32],
            vendor_specific:    [0; 1024],
        }
    }
}

const NVME_IOC_CMD_IDENTIFY: u8 = 0x06;
const NVME_ADMIN_CMD_CNS_ALL_CTRL: u32 = 0x01;

pub struct NvmeController {
    raw_id_data:             NvmeSpecIdCtrlData,
    // Converting u8 array to utf8 might have error which we don't want to
    // trigger during getter function, hence we creat them at struct
    // initialize pharse.
    sn:                         String,
    mn:                         String,
    fr:                         String,
    fguid:                      String,
    subnqn:                     String,
}

fn to_u16(i: [u8; 2]) -> u16 {
    LittleEndian::read_u16(&i)
}

fn to_u32_a3(i: [u8; 3]) -> u32 {
    let mut n = [0u8; 4];
    n[..3].clone_from_slice(&i);
    LittleEndian::read_u32(&n)
}

fn to_u32(i: [u8; 4]) -> u32 {
    LittleEndian::read_u32(&i)
}

impl NvmeController {
    pub fn vid_get(&self)       -> u16  { to_u16(self.raw_id_data.vid) }
    pub fn ssvid_get(&self)     -> u16  { to_u16(self.raw_id_data.ssvid) }
    pub fn sn_get(&self)        -> &str { & self.sn }
    pub fn mn_get(&self)        -> &str { & self.mn }
    pub fn fr_get(&self)        -> &str { & self.fr }
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
    pub fn subnqn_get(&self)    -> &str { & self.subnqn }
    pub fn ioccsz_get(&self)    -> u32 { to_u32(self.raw_id_data.ioccsz) }
    pub fn iorcsz_get(&self)    -> u32 { to_u32(self.raw_id_data.iorcsz) }
    pub fn icdoff_get(&self)    -> u16 { to_u16(self.raw_id_data.icdoff) }
    pub fn ctrattr_get(&self)   -> u8 { self.raw_id_data.ctrattr }
    pub fn msdbd_get(&self)     -> u8 { self.raw_id_data.msdbd }
}

pub fn from_path(blk_path: &str) -> Result<NvmeController> {
    let path =  Path::new(blk_path);

    let fd = OpenOptions::new().read(true).open(&path)?;

    let fd_raw: i32 = AsRawFd::as_raw_fd(&fd);
    let mut id_data: NvmeSpecIdCtrlData = Default::default();

    let nvme_cmd = NvmeAdminCmd {
        opcode:             NVME_IOC_CMD_IDENTIFY,
        addr:               &mut id_data as *mut NvmeSpecIdCtrlData as u64,
        data_len:           size_of::<NvmeAdminCmd>() as u32,
        cdw10:              NVME_ADMIN_CMD_CNS_ALL_CTRL,
        ..                  Default::default()
    };

    unsafe {nvme_ioctl_admin_cmd(fd_raw, &mut [nvme_cmd])?;}
    Ok(
        NvmeController {
            sn:             str::from_utf8(&id_data.sn)?.trim().to_string(),
            mn:             str::from_utf8(&id_data.mn)?.trim().to_string(),
            fr:             str::from_utf8(&id_data.fr)?.trim().to_string(),
            fguid:          str::from_utf8(&id_data.fguid)?.trim().to_string(),
            subnqn:         str::from_utf8(&id_data.subnqn)?.trim().to_string(),
            raw_id_data:    id_data,
        })
}
