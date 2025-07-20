use std::fs;

pub fn import(name: &str) -> Vec<String> {
    fs::read_to_string(name)
        .unwrap()
        .split_terminator('\n')
        .map(|s| s.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
}
