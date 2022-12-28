use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
    let mut count = 0;
    for passport in passports {
        count += (passport
            .keys()
            .filter(|&x| match x {
                b"byr" => true,
                b"iyr" => true,
                b"eyr" => true,
                b"hgt" => true,
                b"hcl" => true,
                b"ecl" => true,
                b"pid" => true,
                b"cid" => false,
                _ => panic!(),
            })
            .count()
            == 7) as usize;
    }
    println!("{}", count);
}
