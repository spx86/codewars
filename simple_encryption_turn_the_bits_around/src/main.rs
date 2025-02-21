use base64::{engine::general_purpose::STANDARD, Engine}; // 确保在字符集范围内

fn swap_c1_5_to_c2_1(c1: u8, c2: u8) -> (u8, u8) {
    let bit_c1 = c1 >> 4 & 1; //提取第五位
    let bit_c2 = c2 >> 7 & 1; //提取第一位

    let byte_c1 = c1 & !(1 << 4) | bit_c2 << 4; //将第五位替换为第一位
    let byte_c2 = c2 & !(1 << 7) | bit_c1 << 7; //将第一位替换为第五位

    (byte_c1, byte_c2)
}

fn inverse_2_and_4_bits(c: u8) -> u8 {
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

fn encrypt(text: &str) -> String {
    if text.is_empty() {
        return text.to_string();
    }

    let mut bytes: Vec<u8> = STANDARD.encode(text.as_bytes()).chars().map(|c| c as u8).collect();

    // 第一步：C1.5 <=> C2.1
    for i in 0..bytes.len() - 1 {
        let (new_c1, new_c2) = swap_c1_5_to_c2_1(bytes[i], bytes[i + 1]);
        bytes[i] = new_c1;
        bytes[i + 1] = new_c2;
    }

    // 逐个字符执行变换
    for i in 0..bytes.len() {
        bytes[i] = inverse_2_and_4_bits(bytes[i]);
        bytes[i] = swap_first_last_3_bits(bytes[i]);
        bytes[i] = swap_odd_even_bits(bytes[i]);
        bytes[i] = reverse_bits(bytes[i]);
        bytes[i] = swap_1_3_bits(bytes[i]);
    }

    STANDARD.encode(&bytes)
}

/// **解密**
fn decrypt(encrypted_text: &str) -> String {
    if encrypted_text.is_empty() {
        return encrypted_text.to_string();
    }

    let mut bytes: Vec<u8> = STANDARD.decode(encrypted_text.as_bytes()).unwrap();

    // 逆向执行变换（与加密顺序相反）
    for i in 0..bytes.len() {
        bytes[i] = swap_1_3_bits(bytes[i]);
        bytes[i] = reverse_bits(bytes[i]);
        bytes[i] = swap_odd_even_bits(bytes[i]);
        bytes[i] = swap_first_last_3_bits(bytes[i]);
        bytes[i] = inverse_2_and_4_bits(bytes[i]);
    }

    // 逆向执行 C1.5 <=> C2.1
    for i in (0..bytes.len() - 1).rev() {
        let (new_c1, new_c2) = swap_c1_5_to_c2_1(bytes[i], bytes[i + 1]);
        bytes[i] = new_c1;
        bytes[i + 1] = new_c2;
    }

    let bytes = String::from_utf8_lossy(&bytes).to_string();
    String::from_utf8(STANDARD.decode(bytes).unwrap()).unwrap()
}

fn main() {
    let text = "afalsgmkaioe23pokd pasodkg  gopasdg kk sdgkospa]kgaspd,vpoaksd g spadk f]ask f";
    println!("原文: {}", text); 
    let encrypted = encrypt(text);
    println!("加密: {}", encrypted);
    let decrypted = decrypt(&encrypted);
    println!("解密: {}", decrypted);
    "sdfad".chars().
    assert_eq!(text, decrypted);
}


