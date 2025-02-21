use std::collections::HashMap;
use std::collections::BTreeSet;

use std::time::Instant;

fn measure_time<F, R>(f: F) -> (R, u128)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();  // 记录开始时间
    let result = f();            // 执行函数
    let duration = start.elapsed().as_micros();  // 计算运行时间（微秒）

    (result, duration)
}

fn factorial(n: u128) -> u128 {
    (1..=n).product()
}

fn alphabetic_anagrams1(s: &str) -> u128 {
    let mut alpha_count = HashMap::new();

    for c in s.chars() {
        *alpha_count.entry(c).or_insert(0) += 1;
    }

    let alpha_str_len = s.len() as u32;
    let mut sorted_chars: Vec<char> = s.chars().collect::<BTreeSet<_>>().into_iter().collect();

    let mut rank = 1; // 排名从 1 开始
    let len = s.len() as u128;
    // 遍历每个字符
    for (i, c) in s.chars().enumerate() {
        // 遍历 sorted_chars 中所有比当前字符小的字符
        for &smaller_char in sorted_chars.iter().filter(|&&ch| ch < c) {
            let mut new_count = alpha_count.clone();
            if let Some(val) = new_count.get_mut(&smaller_char) {
                if *val > 0 {
                    *val -= 1;
                    let denominator: u128 = new_count.values().map(|&v| factorial(v)).product();
                    rank += factorial(len - i as u128 - 1) / denominator;
                }
            }
        }

        // 更新 alpha_count
        if let Some(count) = alpha_count.get_mut(&c) {
            *count -= 1;
        }
    }

    rank
}



fn alphabetic_anagrams(s: &str) -> u128 {
    let mut alpha_count = HashMap::new();

    for c in s.chars() {
        *alpha_count.entry(c).or_insert(0) += 1;
    }

    let alpha_str_len = s.len() as u32;
    let mut sorted_chars: Vec<char> = s.chars().collect::<BTreeSet<_>>().into_iter().collect();

    let len = s.len() as u128;

    s.chars().enumerate().map(|(index, value)| {
        let rank = sorted_chars.iter().filter(|&&ch| ch < value).map(|&smaller_char| {
            let mut new_count = alpha_count.clone();
            if let Some(val) = new_count.get_mut(&smaller_char) {
                if *val > 0 {
                    *val -= 1;
                    let denominator: u128 = new_count.values().map(|&v| factorial(v)).product();
                    factorial(len - index as u128 - 1) / denominator
                } else {
                    0
                }
            } else {
                0
            }
        }).sum::<u128>();
        if let Some(count) = alpha_count.get_mut(&value) {
            *count -= 1;
        } 
        rank
    }).sum::<u128>() + 1
}


fn alphabetic_anagrams2(word: &str) -> u128 {
    let mut pos = 1;  // 当前排列的序号（从1开始）
    let mut perm = 1; // 当前排列的排列数
    let mut cnt = HashMap::<u8, u128>::new();  // 统计字母频率的哈希表
    
    for (i, c1) in word.bytes().rev().enumerate() { // 反向遍历每个字符
        *cnt.entry(c1).or_default() += 1; // 更新字母出现的次数

        // 对已遍历的字母进行字典序比较
        for c2 in cnt.keys() {
            if *c2 < c1 { // 如果某个字符 c2 比当前字符 c1 小，考虑所有可能的排列
                // 通过更新序号来计算当前字符 c1 可能处于字母表中较早位置的排列数
                pos += perm * cnt[&c2] / cnt[&c1];
            }
        }
        
        // 更新排列数 perm，考虑当前字符 c1 出现次数的影响
        perm = perm * (i as u128 + 1) / cnt[&c1];
    }
    
    pos  // 返回计算的排列序号
}


fn alphabetic_anagrams3(word: &str) -> u128 {
    let (mut r, mut s, l, mut c) = (1u128, 1u128, word.len(), HashMap::new());
    for i in 0..l {
        let x = word.chars().nth(l-1-i).unwrap();
        c.entry(x.to_owned()).and_modify(|m|{*m += 1}).or_insert(1);
        let d = c.get(&x).unwrap();
        for (y, e) in &c {
            if y < &&x {
                r += s * e / d;
            }
        }
        s = s * (i+1) as u128 / d;
    }
    r
}


fn main() {
    println!("{}", alphabetic_anagrams("A"));
    println!("{}", alphabetic_anagrams("ABAB"));
    println!("{}", alphabetic_anagrams("AAAB"));
    println!("{}", alphabetic_anagrams("BAAA"));
    println!("{}", alphabetic_anagrams("QUESTION"));
    println!("{}", alphabetic_anagrams("BOOKKEEPER"));
    println!("{}", alphabetic_anagrams("IMMUNOELECTROPHORETICALLY"));
    println!("{}", alphabetic_anagrams("ANTIDISESTABLISHMENTARIANISM"));
    println!("{}", alphabetic_anagrams("FRIENDS"));
    println!("{}", alphabetic_anagrams("FRIEND"));
    println!("{}", alphabetic_anagrams("FRIEN"));
    println!("{}", alphabetic_anagrams("FRIE"));
    println!("{}", alphabetic_anagrams("FRI"));
    println!("{}", alphabetic_anagrams("FR"));
    println!("{}", alphabetic_anagrams("F"));
    println!("{}", alphabetic_anagrams(""));

    let (_, time) = measure_time(|| alphabetic_anagrams1("ANTIDISESTABLISHMENTARIANISM"));
    println!("运行时间: {} 微秒", time);
    let (_, time) = measure_time(|| alphabetic_anagrams("ANTIDISESTABLISHMENTARIANISM"));
    println!("运行时间: {} 微秒", time);
    let (_, time) = measure_time(|| alphabetic_anagrams2("ANTIDISESTABLISHMENTARIANISM"));
    println!("运行时间: {} 微秒", time);
    let (_, time) = measure_time(|| alphabetic_anagrams3("ANTIDISESTABLISHMENTARIANISM"));
    println!("运行时间: {} 微秒", time);
}
