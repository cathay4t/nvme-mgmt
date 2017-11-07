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

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub(crate) struct NvmeSpecPsd {
    mp:                             [u8; 2],
    reserved_0:                     u8,
    _bit_field_0:                   u8,        /* mxps:1, nops:1, reserve:6 */
    enlat:                          [u8; 4],
    exlat:                          [u8; 4],
    _bit_field_1:                   u8,        /* rrt:5, reserve:3 */
    _bit_field_2:                   u8,        /* rrl:5, reserved:3 */
    _bit_field_3:                   u8,        /* rwt:5, reserved:3 */
    _bit_field_4:                   u8,        /* rwl:5, reserved:3 */
    idlp:                           [u8; 2],
    _bit_field_5:                   u8,        /* reserved:6, ips:2 */
    reserved_7:                     u8,
    actp:                           [u8; 2],
    _bit_field_6:                   u8,        /* apw:3, reserved:3, aps:2 */
    reserved_9:                     [u8; 9],
}

impl Default for NvmeSpecPsd {
    fn default() -> NvmeSpecPsd {
        NvmeSpecPsd {
            mp:                     [0; 2],
            reserved_0:             0,
            _bit_field_0:           0,
            enlat:                  [0; 4],
            exlat:                  [0; 4],
            _bit_field_1:           0,
            _bit_field_2:           0,
            _bit_field_3:           0,
            _bit_field_4:           0,
            idlp:                   [0; 2],
            _bit_field_5:           0,
            reserved_7:             0,
            actp:                   [0; 2],
            _bit_field_6:           0,
            reserved_9:             [0; 9],
        }
    }
}

#[repr(C, packed)]
pub(crate) struct NvmeSpecIdCtrlData {
    pub(crate) vid:                 [u8; 2],
    pub(crate) ssvid:               [u8; 2],
    pub(crate) sn:                  [u8; 20],
    pub(crate) mn:                  [u8; 40],
    pub(crate) fr:                  [u8; 8],
    pub(crate) rab:                 u8,
    pub(crate) ieee:                [u8; 3],
    pub(crate) cmic:                u8,
    pub(crate) mdts:                u8,
    pub(crate) cntlid:              [u8; 2],
    pub(crate) ver:                 [u8; 4],
    pub(crate) rtd3r:               [u8; 4],
    pub(crate) rtd3e:               [u8; 4],
    pub(crate) oaes:                [u8; 4],
    pub(crate) ctratt:              [u8; 4],
    pub(crate) reserved_0:          [u8; 12],
    pub(crate) fguid:               [u8; 16],
    pub(crate) reserved_1:          [u8; 112],
    pub(crate) ressered_mi:         [u8; 16],
    pub(crate) oacs:                [u8; 2],
    pub(crate) acl:                 u8,
    pub(crate) aerl:                u8,
    pub(crate) frmw:                u8,
    pub(crate) lpa:                 u8,
    pub(crate) elpe:                u8,
    pub(crate) npss:                u8,
    pub(crate) avscc:               u8,
    pub(crate) apsta:               u8,
    pub(crate) wctemp:              [u8; 2],
    pub(crate) cctemp:              [u8; 2],
    pub(crate) mtfa:                [u8; 2],
    pub(crate) hmpre:               [u8; 4],
    pub(crate) hmmin:               [u8; 4],
    pub(crate) tnvmcap:             [u8; 16],
    pub(crate) unvmcap:             [u8; 16],
    pub(crate) rpmbs:               [u8; 4],
    pub(crate) edstt:               [u8; 2],
    pub(crate) esto:                u8,
    pub(crate) fwug:                u8,
    pub(crate) kas:                 [u8; 2],
    pub(crate) hctma:               [u8; 2],
    pub(crate) mntmt:               [u8; 2],
    pub(crate) mxtmt:               [u8; 2],
    pub(crate) sanicap:             [u8; 4],
    pub(crate) reserved_2:          [u8; 180],
    pub(crate) sqes:                u8,
    pub(crate) cqes:                u8,
    pub(crate) maxcmd:              [u8; 2],
    pub(crate) nn:                  [u8; 4],
    pub(crate) oncs:                [u8; 2],
    pub(crate) fuses:               [u8; 2],
    pub(crate) fna:                 u8,
    pub(crate) vwc:                 u8,
    pub(crate) awun:                [u8; 2],
    pub(crate) awupf:               [u8; 2],
    pub(crate) nvscc:               u8,
    pub(crate) reserved_3:          u8,
    pub(crate) acwu:                [u8; 2],
    pub(crate) reserved_4:          [u8; 2],
    pub(crate) sgls:                [u8; 4],
    pub(crate) reserved_5:          [u8; 228],
    pub(crate) subnqn:              [u8; 256],
    pub(crate) reserved_6:          [u8; 768],
    /* Below are for NVMe Fabric */
    pub(crate) ioccsz:              [u8; 4],
    pub(crate) iorcsz:              [u8; 4],
    pub(crate) icdoff:              [u8; 2],
    pub(crate) ctrattr:             u8,
    pub(crate) msdbd:               u8,
    pub(crate) reserved_7:          [u8; 244],
    /* Above are for NVMe Fabric */
    pub(crate) psds:                [NvmeSpecPsd; 32],
    pub(crate) vendor_specific:     [u8; 1024],
}

impl Default for NvmeSpecIdCtrlData {
    fn default() -> NvmeSpecIdCtrlData {
        NvmeSpecIdCtrlData {
            vid:                    [0; 2],
            ssvid:                  [0; 2],
            sn:                     [0; 20],
            mn:                     [0; 40],
            fr:                     [0; 8],
            rab:                    0,
            ieee:                   [0; 3],
            cmic:                   0,
            mdts:                   0,
            cntlid:                 [0; 2],
            ver:                    [0; 4],
            rtd3r:                  [0; 4],
            rtd3e:                  [0; 4],
            oaes:                   [0; 4],
            ctratt:                 [0; 4],
            reserved_0:             [0; 12],
            fguid:                  [0; 16],
            reserved_1:             [0; 112],
            ressered_mi:            [0; 16],
            oacs:                   [0; 2],
            acl:                    0,
            aerl:                   0,
            frmw:                   0,
            lpa:                    0,
            elpe:                   0,
            npss:                   0,
            avscc:                  0,
            apsta:                  0,
            wctemp:                 [0; 2],
            cctemp:                 [0; 2],
            mtfa:                   [0; 2],
            hmpre:                  [0; 4],
            hmmin:                  [0; 4],
            tnvmcap:                [0; 16],
            unvmcap:                [0; 16],
            rpmbs:                  [0; 4],
            edstt:                  [0; 2],
            esto:                   0,
            fwug:                   0,
            kas:                    [0; 2],
            hctma:                  [0; 2],
            mntmt:                  [0; 2],
            mxtmt:                  [0; 2],
            sanicap:                [0; 4],
            reserved_2:             [0; 180],
            sqes:                   0,
            cqes:                   0,
            maxcmd:                 [0; 2],
            nn:                     [0; 4],
            oncs:                   [0; 2],
            fuses:                  [0; 2],
            fna:                    0,
            vwc:                    0,
            awun:                   [0; 2],
            awupf:                  [0; 2],
            nvscc:                  0,
            reserved_3:             0,
            acwu:                   [0; 2],
            reserved_4:             [0; 2],
            sgls:                   [0; 4],
            reserved_5:             [0; 228],
            subnqn:                 [0; 256],
            reserved_6:             [0; 768],
            ioccsz:                 [0; 4],
            iorcsz:                 [0; 4],
            icdoff:                 [0; 2],
            ctrattr:                0,
            msdbd:                  0,
            reserved_7:             [0; 244],
            psds:                   [Default::default(); 32],
            vendor_specific:        [0; 1024],
        }
    }
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub(crate) struct NvmeSpecLbaf {
    pub(crate) ms:                  [u8; 2],
    pub(crate) lbads:               u8,
    pub(crate) _bit_field_0:        u8,
}

impl Default for NvmeSpecLbaf {
    fn default() -> NvmeSpecLbaf {
        NvmeSpecLbaf {
            ms:                     [0, 2],
            lbads:                  0,
            _bit_field_0:           0,
        }
    }
}

#[repr(C, packed)]
pub(crate) struct NvmeSpecIdNsData {
    pub(crate) nsze:                [u8; 8],
    pub(crate) ncap:                [u8; 8],
    pub(crate) nuse:                [u8; 8],
    pub(crate) nsfeat:              u8,
    pub(crate) nlbaf:               u8,
    pub(crate) flbas:               u8,
    pub(crate) mc:                  u8,
    pub(crate) dpc:                 u8,
    pub(crate) dps:                 u8,
    pub(crate) nmic:                u8,
    pub(crate) rescap:              u8,
    pub(crate) fpi:                 u8,
    pub(crate) elfeat:              u8,
    pub(crate) nawun:               [u8; 2],
    pub(crate) nawupf:              [u8; 2],
    pub(crate) nacwu:               [u8; 2],
    pub(crate) nabsn:               [u8; 2],
    pub(crate) nabo:                [u8; 2],
    pub(crate) nabspf:              [u8; 2],
    pub(crate) noiob:               [u8; 2],
    pub(crate) nvmcap:              [u8; 16],
    pub(crate) reserved_0:          [u8; 40],
    pub(crate) nguid:               [u8; 16],
    pub(crate) eui64:               [u8; 8],
    pub(crate) lbafs:               [NvmeSpecLbaf; 16],
    pub(crate) reserved_1:          [u8; 192],
    pub(crate) vendor_specific:     [u8; 3712],
}

