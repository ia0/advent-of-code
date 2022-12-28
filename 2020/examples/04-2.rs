use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader};

use lazy_static::lazy_static;

lazy_static! {
    static ref COLORS: HashSet<&'static [u8; 3]> =
        [b"amb", b"blu", b"brn", b"gry", b"grn", b"hzl", b"oth"].iter().cloned().collect();
}

fn entry_is_valid(key: &[u8; 3], value: &[u8]) -> bool {
    match key {
        b"byr" => {
            let value = std::str::from_utf8(value).unwrap().parse().unwrap();
            1920 <= value && value <= 2002
        }
        b"iyr" => {
            let value = std::str::from_utf8(value).unwrap().parse().unwrap();
            2010 <= value && value <= 2020
        }
        b"eyr" => {
            let value = std::str::from_utf8(value).unwrap().parse().unwrap();
            2020 <= value && value <= 2030
        }
        b"hgt" => {
            if let Some(value) = value.strip_suffix(b"cm") {
                let value = std::str::from_utf8(value).unwrap().parse().unwrap();
                150 <= value && value <= 193
            } else if let Some(value) = value.strip_suffix(b"in") {
                let value = std::str::from_utf8(value).unwrap().parse().unwrap();
                59 <= value && value <= 76
            } else {
                false
            }
        }
        b"hcl" => {
            value.len() == 7
                && value[0] == b'#'
                && value[1 ..].iter().all(|x| x.is_ascii_hexdigit() && !x.is_ascii_uppercase())
        }
        b"ecl" => match <&[u8; 3]>::try_from(value) {
            Ok(value) => COLORS.contains(value),
            Err(_) => false,
        },
        b"pid" => value.len() == 9 && value.iter().all(|x| x.is_ascii_digit()),
        b"cid" => false,
        _ => panic!(),
    }
}

fn passport_is_valid(passport: &HashMap<[u8; 3], Vec<u8>>) -> bool {
    passport.iter().filter(|(k, v)| entry_is_valid(k, v)).count() == 7
}

fn main() {
    let input = File::open("examples/04.txt").unwrap();
    let mut passports = vec![HashMap::new()];
    for line in BufReader::new(input).lines() {
        let line = line.unwrap().into_bytes();
        if line.is_empty() {
            passports.push(HashMap::new());
            continue;
        }
        for kv in line.split(|&x| x == b' ') {
            assert_eq!(kv[3], b':');
            let k = <[u8; 3]>::try_from(&kv[.. 3]).unwrap();
            let v = kv[4 ..].to_vec();
            assert!(passports.last_mut().unwrap().insert(k, v).is_none());
        }
    }
    println!("{}", passports.iter().filter(|p| passport_is_valid(p)).count());
}
