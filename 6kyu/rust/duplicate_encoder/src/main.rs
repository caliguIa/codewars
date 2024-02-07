use std::collections::HashMap;

fn main() {}

fn duplicate_encode(word: &str) -> String {
    let mut counter = HashMap::<String, u32>::new();
    let mut char_vec = Vec::new();
    let formatted_word = word.to_lowercase();

    for l in formatted_word.chars() {
        let cur_val = counter.get(&String::from(l)).copied().unwrap_or(0);
        counter.insert(String::from(l), cur_val + 1);
    }

    for l in formatted_word.chars() {
        let cur_val = counter.get(&String::from(l)).copied().unwrap_or(0);
        if cur_val > 1 {
            char_vec.push(")");
        } else {
            char_vec.push("(");
        };
    }

    return char_vec.into_iter().collect();
}

#[cfg(test)]
mod tests {
    use crate::duplicate_encode;
    #[test]
    fn run_tests() {
        assert_eq!(duplicate_encode("din"), "(((");
        assert_eq!(duplicate_encode("recede"), "()()()");
        assert_eq!(duplicate_encode("Success"), ")())())", "should ignore case");
        assert_eq!(duplicate_encode("(( @"), "))((");
    }
}
