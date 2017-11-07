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
//#[macro_use] extern crate prettytable;

/*
 * /dev/nvme0n1 '$vendor' '$modele' '$capacity' '$health'
 */

fn main() {
    let ctrls = nvme::NvmeController::get_all().unwrap();
    for c in ctrls {
        println!("{}:", c.blk_path_get());
        println!("\tVID: '{}'", c.vid_get());
        println!("\tSN: '{}'", c.sn_get());
        println!("\tMN: '{}'", c.mn_get());
        println!("\tFR: '{}'", c.fr_get());
        println!("\tRAB: '{}'", c.rab_get());
        println!("\tIEEE: '{}'", c.ieee_get());
        println!("\tFGUID: '{}'", c.fguid_get());
        println!("\tSUBNQN: '{}'", c.subnqn_get());
        println!("\tCNTLID: '{}'", c.cntlid_get());
        println!("\tVER: '{}'", c.ver_str_get());
        println!("\tNN: '{}'", c.nn_get());
        println!("\tCQES: '{}'", c.cqes_get());
    }
}
