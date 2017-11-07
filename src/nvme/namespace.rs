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

use super::ioctl::nvme_ioctl_admin_cmd;
use super::spec::NvmeSpecIdNsData;
use super::error::*;
use super::NvmeController;

pub struct NvmeNameSpace {
    raw_id_data:                NvmeSpecIdNsData,
    // Converting u8 array to utf8 might have error which we don't want to
    // trigger during getter function, hence we creat them at struct
    // initialize pharse.
    blk_path:                   String,
}

impl NvmeNameSpace {
    pub(crate) fn new(blk_path: &str, ctrl: &NvmeController,
                      ns_id: u32) -> Result<NvmeNameSpace> {
        Err(ErrorKind::LibBug("Coding".to_string()).into())
    }

    pub(crate) fn ns_id_list_get(blk_path: &str,
                                 ctrl: &NvmeController) -> Result<Vec<u32>> {
        let mut ret = Vec::new();
        if ctrl.ver_get() < NvmeController::ver_gen(1, 1, 0) {
            /* pre SPEC 1.1.0, Namespaces shall be allocated in
             * order (starting with 1) and packed sequentially.
             */
            for ns_id in 1..ctrl.nn_get() + 1 {
                ret.push(ns_id);
            }
        }
        Ok(ret)
    }
}
