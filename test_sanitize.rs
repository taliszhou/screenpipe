fn main() {
    let q = "-verify";
    let cleaned = q.split_whitespace().filter_map(|t| {
        let cleaned = t.replace(['\\', '"'], "");
        if cleaned.is_empty() { return None; }
        Some(format!("\"{}\"", cleaned))
    }).collect::<Vec<_>>().join(" ");
    println!("Cleaned: {}", cleaned);
}
