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

/*
 * /dev/nvme0 '$vendor' '$modele' '$capacity' '$health'
 * \t$nsid /dev/nvme0n1 '$capacity'
 */

fn main() {
    let c = nvme::controller::from_path("/dev/nvme0").unwrap();
    println!("VID: '{}'", c.vid_get());
    println!("SN: '{}'", c.sn_get());
    println!("MN: '{}'", c.mn_get());
    println!("FR: '{}'", c.fr_get());
    println!("RAB: '{}'", c.rab_get());
    println!("IEEE: '{}'", c.ieee_get());
    println!("FGUID: '{}'", c.fguid_get());
    println!("SUBNQN: '{}'", c.subnqn_get());
}
