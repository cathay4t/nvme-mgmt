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
extern crate nvme;
#[macro_use] extern crate prettytable;

use nvme::NvmeController;
use prettytable::Table;
use prettytable::format::consts::FORMAT_CLEAN;

/*
 * /dev/nvme0n1 '$modele' '$capacity' 'temp' '$health'
 */

struct DisplayEntry {
    blk_path:       String,
    wwid:           String,
    firmware:       String,
    model:          String,
    size:           String,
}

fn quick_info_display(quick_info: Vec<DisplayEntry>) {
    let mut table = Table::new();

    table.set_format(*FORMAT_CLEAN);
    for entry in quick_info {
        table.add_row(row![entry.blk_path, entry.wwid, entry.model,
                           entry.firmware, entry.size]);
    }

    table.printstd();
}

fn main() {
    let ctrls = NvmeController::get_all().unwrap();
    let mut quick_info: Vec<DisplayEntry> = Vec::new();
    for c in ctrls {
        let nss = c.namespaces_get().unwrap();
        for ns in nss {
            let mut wwid = ns.nguid_get();
            if wwid.len() == 0 {
                wwid = ns.eui64_get();
            }
            if wwid.len() == 0 {
                wwid = "0000000000000000";
            }
            quick_info.push(DisplayEntry{
                blk_path:   format!("{}", ns.blk_path_get()),
                model:      format!("{}", c.mn_get()),
                size:       nvme::size_bytes_2_size_human(ns.size_get()),
                wwid:       wwid.to_string(),
                firmware:   format!("{}", c.fr_get()),
            });
        }
    }
    quick_info_display(quick_info);
}
