pub fn truncate(v: &[u8], max_chars_count: usize) -> String {
    let max_bytes_count = 4 * max_chars_count;
    let max_bytes_count = match v.len() <= max_bytes_count {
        true => v.len(),
        _ => max_bytes_count,
    };

    let newstr: String = String::from_utf8_lossy(&v[..max_bytes_count])
        .chars()
        .take(max_chars_count)
        .collect();
    match v.len() == newstr.len() {
        true => newstr,
        _ => format!("{}... (truncated)", &newstr),
    }
}
