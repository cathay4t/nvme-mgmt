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

fn main() {
    let c = nvme::controller::from_path("/dev/nvme0").unwrap();
    println!("VID: '{}'", c.get_vid());
    println!("SN: '{}'", c.get_sn());
    println!("MN: '{}'", c.get_mn());
    println!("FR: '{}'", c.get_fr());
    println!("RAB: '{}'", c.get_rab());
    println!("IEEE: '{}'", c.get_ieee());
    println!("FGUID: '{}'", c.get_fguid());
    println!("SUBNQN: '{}'", c.get_subnqn());
}
