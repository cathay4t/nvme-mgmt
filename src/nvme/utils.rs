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

use byteorder::{ByteOrder, LittleEndian};

pub(crate) fn to_u16(i: [u8; 2]) -> u16 {
    LittleEndian::read_u16(&i)
}

pub(crate) fn u24_to_u32(i: [u8; 3]) -> u32 {
    LittleEndian::read_u24(&i)
}

pub(crate) fn to_u32(i: [u8; 4]) -> u32 {
    LittleEndian::read_u32(&i)
}

pub(crate) fn to_u64(i: [u8; 8]) -> u64 {
    LittleEndian::read_u64(&i)
}

pub(crate) fn bit_field_extract(i: u8, end_include: u8,
                                start_include: u8) -> u8 {
    (i >> start_include) & ((1 << (end_include - start_include + 1)) - 1)
}

struct SizeUnit<'a> {
    unit:   &'a str,
    bytes:  u64,
}

const SIZE_CONVS:[SizeUnit<'static>; 6] = [
    SizeUnit{unit: "EiB", bytes: 1u64 << 60},
    SizeUnit{unit: "PiB", bytes: 1u64 << 50},
    SizeUnit{unit: "TiB", bytes: 1u64 << 40},
    SizeUnit{unit: "GiB", bytes: 1u64 << 30},
    SizeUnit{unit: "MiB", bytes: 1u64 << 20},
    SizeUnit{unit: "KiB", bytes: 1u64 << 10},
];

pub fn size_bytes_2_size_human(i: u64) -> String {
    let mut unit = "B";
    let mut num:f64 = 0f64;
    for size_conv in SIZE_CONVS.iter() {
        if i >= size_conv.bytes {
            num = (i as f64) / (size_conv.bytes as f64);
            unit = size_conv.unit;
            break;
        }
    }
    if num == 0f64 {
        num = i as f64;
    }
    format!("{:.2}{}", num, unit)
}

pub(crate) fn to_hex_string(data: &[u8]) -> String {
    let mut all_zero = true;
    let mut ret = String::new();
    for i in data {
        if *i != 0 {
            all_zero = false;
        }
        ret.push_str(&format!("{:02x}", i));
    }
    match all_zero {
        true => String::new(),
        false => ret
    }
}
