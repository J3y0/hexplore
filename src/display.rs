pub fn format_hex(bytes: &[u8], padding: usize) -> String {
    let mut s = String::with_capacity(3 * bytes.len());

    for b in bytes.iter() {
        s.push_str(format!("{b:02X} ").as_str());
    }
    // Pop last space
    s.pop();

    for _ in 0..padding {
        s.push_str("   ");
    }

    s
}

pub fn format_ascii(bytes: &[u8], padding: usize) -> String {
    let mut s = String::with_capacity(bytes.len());

    for b in bytes.iter() {
        if b.is_ascii_graphic() {
            s.push((*b).into());
        } else {
            s.push('.');
        }
    }

    for _ in 0..padding {
        s.push(' ');
    }

    s
}

pub fn format_index(i: usize, cap: usize) -> String {
    let mut s = String::with_capacity(cap);
    let hex = format!("{i:x}");
    for _ in hex.len()..cap {
        s.push('0');
    }
    s.push_str(&hex);

    s
}
