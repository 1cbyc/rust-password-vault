use rand::Rng;
pub fn generate(length: usize, symbols: bool, uppercase: bool, lowercase: bool, numbers: bool) -> String {
    let mut charset = String::new();
    if lowercase { charset.push_str("abcdefghijklmnopqrstuvwxyz"); }
    if uppercase { charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ"); }
    if numbers { charset.push_str("0123456789"); }
    if symbols { charset.push_str("!@#$%^&*()-_=+[]{};:,.<>?/|\\"); }
    if charset.is_empty() { charset.push_str("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"); }
    let mut rng = rand::thread_rng();
    (0..length).map(|_| {
        let idx = rng.gen_range(0..charset.len());
        charset.chars().nth(idx).unwrap()
    }).collect()
} 