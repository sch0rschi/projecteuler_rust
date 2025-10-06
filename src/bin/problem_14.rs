use std::collections::HashMap;

fn main() {
    let mut chaining_counts: HashMap<i64, i64> = HashMap::new();
    chaining_counts.insert(1, 1);
    let mut max_chaining_count = 0;
    let mut number_having_max_chain = 1;
    for i in 2..1_000_000 {
        let chaining_number = find_chaining_number_recursion(i, &mut chaining_counts);
        if chaining_number > max_chaining_count {
            max_chaining_count = chaining_number;
            number_having_max_chain = i;
        }
    }
    println!("{}", number_having_max_chain);
}

fn find_chaining_number_recursion(n: i64, cache: &mut HashMap<i64, i64>) -> i64 {
    if cache.contains_key(&n) {
        return *cache.get(&n).unwrap();
    }
    if n % 2 == 0 {
        let chaining_number_step = find_chaining_number_recursion(n / 2, cache);
        cache.insert(n, chaining_number_step + 1);
        chaining_number_step + 1
    } else {
        let chaining_number_step = find_chaining_number_recursion(3 * n + 1, cache);
        cache.insert(n, chaining_number_step + 1);
        chaining_number_step + 1
    }
}
