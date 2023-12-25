use std::collections::HashMap;

pub async fn get_gift_emojis(numbers: String) -> String {
    let counter = numbers
        .trim()
        .lines()
        .fold(HashMap::<&str, usize>::new(), |mut counter, l| {
            counter.entry(l).and_modify(|e| *e += 1).or_insert(1);
            counter
        });
    let n = counter
        .iter()
        .filter(|(_, v)| **v < 2)
        .flat_map(|(k, _)| k.parse::<u64>().ok())
        .take(1)
        .collect::<Vec<_>>();

    if n.is_empty() {
        return String::new();
    }

    (0..n[0]).map(|_| "🎁").collect::<String>()
}
