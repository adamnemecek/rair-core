/*
 * print_hex: commands handling hex printing.
 * Copyright (C) 2019  Oddcoder
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use core::*;
use helper::*;
use std::cmp;
use std::io::Write;
use writer::*;
use yansi::Paint;

#[derive(Default)]
pub struct PrintHex {}

impl PrintHex {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Cmd for PrintHex {
    fn run(&mut self, core: &mut Core, args: &[String]) {
        if args.len() != 1 {
            expect(core, args.len() as u64, 1);
            return;
        }
        let loc = core.get_loc();
        let size;
        match str_to_num(&args[0]) {
            Ok(s) => size = s,
            Err(e) => {
                error_msg(
                    core,
                    &e.to_string(),
                    &format!("Expect Hex, binary, Octal or Decimal value but found {} instead", Paint::default(&args[0]).italic()),
                );
                return;
            }
        }
        let data_or_no_data = match core.mode {
            AddrMode::Phy => core.io.pread_sparce(loc, size),
            AddrMode::Vir => core.io.vread_sparce(loc, size),
        };
        let data;
        match data_or_no_data {
            Ok(d) => data = d,
            Err(e) => {
                error_msg(core, "Read Failed", &e.to_string());
                return;
            }
        }
        let banner = core.color_palette[5];
        let na = core.color_palette[4];
        writeln!(
            core.stdout,
            "{}",
            Paint::rgb(banner.0, banner.1, banner.2, "- offset -  0 1  2 3  4 5  6 7  8 9  A B  C D  E F  0123456789ABCDEF")
        )
        .unwrap();
        for i in (0..size).step_by(16) {
            write!(core.stdout, "{} ", Paint::rgb(banner.0, banner.1, banner.2, format!("0x{:08x}", loc + i))).unwrap();
            let mut ascii = Writer::new_buf();
            let mut hex = Writer::new_buf();
            for j in i..cmp::min(i + 16, size) {
                if let Some(c) = data.get(&(j + loc)) {
                    if j % 2 == 0 {
                        write!(hex, "{:02x}", c).unwrap();
                    } else {
                        write!(hex, "{:02x} ", c).unwrap();
                    }
                    if *c >= 0x21 && *c <= 0x7E {
                        write!(ascii, "{}", *c as char).unwrap()
                    } else {
                        write!(ascii, "{}", Paint::rgb(na.0, na.1, na.2, ".")).unwrap();
                    }
                } else {
                    if j % 2 == 0 {
                        write!(hex, "**").unwrap();
                    } else {
                        write!(hex, "** ").unwrap();
                    }
                    write!(ascii, "{}", Paint::rgb(na.0, na.1, na.2, "*")).unwrap();
                }
            }
            writeln!(core.stdout, "{: <40} {}", hex.utf8_string().unwrap(), ascii.utf8_string().unwrap()).unwrap();
        }
    }
    fn help(&self, core: &mut Core) {
        help(core, &"printHex", &"px", vec![("[size]", "View data of at current location in hex format")]);
    }
}
