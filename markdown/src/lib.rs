pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

macro_rules! file_path {
    ($($key:expr => $value:expr),*) => {
        {
            vec![
                $(
                    ($key, include_str!($value)),
                )*
            ]
        }
    }
}
