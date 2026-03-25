pub fn to_vec2d<T: Clone, const N: usize>(data: impl IntoIterator<Item = [T; N]>) -> Vec<Vec<T>> {
    data.into_iter().map(|row| row.to_vec()).collect()
}

pub fn to_char_vec<'a>(data: impl IntoIterator<Item = &'a str>) -> Vec<char> {
    data.into_iter()
        .map(|s| s.chars().next().unwrap())
        .collect()
}

pub fn to_string_vec<'a>(data: impl IntoIterator<Item = &'a str>) -> Vec<String> {
    data.into_iter().map(|s| s.to_string()).collect()
}
