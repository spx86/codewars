use std::collections::HashMap;

fn is_valid_walk(walk: &[char]) -> bool {
    if walk.len() != 10 {
        return false;
    }

    let mut char_count = HashMap::new();

    walk.iter().for_each(|c| {
        *char_count.entry(c).or_insert(0) += 1;
    });

    if char_count.get(&'n') == char_count.get(&'s') && char_count.get(&'w') == char_count.get(&'e') {
        return true;
    }
    false
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_walk() {
        assert_eq!(is_valid_walk(&['n', 's', 'n', 's', 'n', 's', 'n', 's', 'n', 's']), true);
        assert_eq!(is_valid_walk(&['w', 'e', 'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e']), true);
        assert_eq!(is_valid_walk(&['w', 'e']), false);
        assert_eq!(is_valid_walk(&['n', 'n', 'n', 's', 'n', 's', 'n', 's', 'n', 's']), false);
    }
}