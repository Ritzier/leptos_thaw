mod md;

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
