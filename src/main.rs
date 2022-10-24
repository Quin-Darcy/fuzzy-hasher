use rand;
use std::io::Read;
use std::fs::File;
use std::io::BufReader;
use rayon::prelude::*;


const HD_THRESHOLD: usize = 461;
const NUM_OF_BBLOCKS: usize = 16;
const BBLOCK_BYTE_LEN: usize = 128;

fn hamming_dist(bytes1: &Vec<u8>, bytes2: &Vec<u8>) -> u32 {
    let mut d: u32 = 0;
    for i in 0..bytes1.len() {
        let bin_str: Vec<char> = format!("{:b}", bytes1[i]^bytes2[i]).chars().collect();
        let bin_str_reduced: Vec<char> = bin_str.into_iter().filter(|&x| x == '1').collect();
        d += bin_str_reduced.len() as u32;
    }
    d
}

fn get_bblocks() -> Vec<Vec<u8>> {
    let mut bblocks: Vec<Vec<u8>> = Vec::new();

    for _ in 0..NUM_OF_BBLOCKS {
        bblocks.push((0..BBLOCK_BYTE_LEN).map(|_| { rand::random::<u8>() }).collect());
    }
    bblocks
}

fn get_message_bytes(d_type: u32, data: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut msg_bytes: Vec<u8> = Vec::new();

    if d_type == 102 {
        let file = match File::open(data) {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        let mut reader = BufReader::new(file);
        reader.read_to_end(&mut msg_bytes).unwrap();
    } else {
        msg_bytes = data.as_bytes().to_vec();
    }
    Ok(msg_bytes)
}

fn get_fuzzy_hash(bblocks: Vec<Vec<u8>>, msg_bytes: Vec<u8>) -> String {
    let mut hash: String = String::new();
    let msg_blocks: Vec<Vec<u8>> = (0..msg_bytes.len()-BBLOCK_BYTE_LEN-1)
        .into_par_iter()
        .map(|i| msg_bytes[i..i+BBLOCK_BYTE_LEN].to_vec())
        .collect();

        for block in msg_blocks {
            let mut current_dist;
            let mut min_index: usize = 0;
            let mut min_d: u32 = 8 * BBLOCK_BYTE_LEN as u32;

            for j in 0..NUM_OF_BBLOCKS {
                current_dist = hamming_dist(&block, &bblocks[j]);
                if current_dist < min_d {
                    min_d = current_dist;
                    min_index = j;
                }
            }
            if min_d < HD_THRESHOLD as u32 {
                hash.push_str(&format!("{:0x}", min_index)[..]);
            }

    hash
}

fn main() {
    let path: &str = "/home/arbegla/Downloads/voronoi.png";
    let bblocks: Vec<Vec<u8>> = get_bblocks();
    let msg_bytes: Vec<u8> = get_message_bytes(102, path).unwrap();
    let hash1: String = get_fuzzy_hash(bblocks, msg_bytes);
    println!("{:?}", &hash1);
}
