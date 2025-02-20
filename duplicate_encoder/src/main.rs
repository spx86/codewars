use std::time::Instant;
use std::collections::HashMap;

fn measure_time<F, R>(f: F) -> (R, u128)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();  // 记录开始时间
    let result = f();            // 执行函数
    let duration = start.elapsed().as_micros();  // 计算运行时间（微秒）

    (result, duration)
}


fn duplicate_encode(word:&str)->String {
    let word = word.to_lowercase();
    word.chars()
        .map(|c| if word.chars()
                                .filter(|x| *x == c)
                                .count() > 1 {')'} 
                        else {'('})
                        .collect()
}

fn duplicate_encode1(word: &str) -> String {
    let lower_word = word.to_lowercase(); // 统一转换为小写
    let mut char_count = HashMap::new(); // 创建哈希表存储字符频率

    // 计算字符出现次数
    for c in lower_word.chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }

    // 构造输出字符串
    lower_word.chars()
        .map(|c| if char_count[&c] > 1 { ')' } else { '(' })
        .collect()
}

fn main() {
    let str = "In practice, for an application that needs to perform some computation F on data that is encrypted, the FHE scheme would provide some alternative computation F' which when applied directly over the encrypted data will result in the encryption of the application of F over the data in the clear. More formally: F(unencrypted_data) = Decrypt(F'(encrypted_data))";

    let (_, time) = measure_time(|| duplicate_encode(str));
    println!("运行时间: {} 微秒", time);
    let (_, time) = measure_time(|| duplicate_encode1(str));
    println!("运行时间: {} 微秒", time);
    
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn run_tests() {
    assert_eq!(duplicate_encode("din"),"(((");
    assert_eq!(duplicate_encode("recede"),"()()()");
    assert_eq!(duplicate_encode("Success"),")())())","should ignore case");
    assert_eq!(duplicate_encode("(( @"),"))((");
    }
}