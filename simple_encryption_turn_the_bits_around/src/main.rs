#![feature(iter_map_windows)]
fn swap_c1_5_to_c2_1(c1: u8, c2: u8) -> (u8, u8) {
    let bit_c1 = c1 >> 4 & 1; //提取第五位
    let bit_c2 = c2 >> 7 & 1; //提取第一位

    let byte_c1 = c1 & !(1 << 4) | bit_c2 << 4; //将第五位替换为第一位
    let byte_c2 = c2 & !(1 << 7) | bit_c1 << 7; //将第一位替换为第五位

    (byte_c1, byte_c2)
}

fn inverse_c_2_and_4(c: u8) -> u8 {
    let c = c ^ 1 << 6;
    let c = c ^ 1 << 3;
    c
}

fn swap_first_last_3_bits(c: u8) -> u8 {
    let first_3_bits = c >> 5 & 0b111;
    let last_3_bits = c & 0b111;

    let c = c & !(0b111 << 5) | last_3_bits << 5;
    let c = c & !0b111 | first_3_bits;
    c
}

fn swap_odd_even_bits(c: u8) -> u8 {
    let odd_bits = c & 0b1010_1010;
    let even_bits = c & 0b0101_0101;

    let odd_bits = odd_bits >> 1;
    let even_bits = even_bits << 1;

    odd_bits | even_bits
}

fn reverse_bits(c: u8) -> u8 {
    let mut c = c;
    c = (c & 0b1010_1010) >> 1 | (c & 0b0101_0101) << 1;
    c = (c & 0b1100_1100) >> 2 | (c & 0b0011_0011) << 2;
    c = (c & 0b1111_0000) >> 4 | (c & 0b0000_1111) << 4;
    c
}

fn swap_1_3_bits(c: u8) -> u8 {
    let bit_1 = c >> 7 & 1;
    let bit_3 = c >> 5 & 1;

    let c = c & !(1 << 7) | bit_3 << 7;
    let c = c & !(1 << 5) | bit_1 << 5;
    c
}
fn main() {
    let (x, y) = swap_c1_5_to_c2_1('H' as u8, 'e' as u8);
    println!("H:{:08b}, swap:{:08b}", 'H' as u8,  x);
    println!("e:{:08b}, swap:{:08b}", 'e' as u8,  y);

    let s = "Hello, world".to_string();
    println!("{:?}", s.as_bytes());
    let s = s.bytes().map_windows(|[x, y]| {
        let (c1, c2) = swap_c1_5_to_c2_1(*x, *y);
        (c1, c2)
    }
    ).collect::<Vec<_>>();
    //println!("{:?}", String::from_utf8_lossy(&s).to_string());
    println!("{:?}", s);
}
