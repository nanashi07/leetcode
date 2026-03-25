pub fn to_vec2d<T: Clone, const N: usize>(data: impl IntoIterator<Item = [T; N]>) -> Vec<Vec<T>> {
    data.into_iter().map(|row| row.to_vec()).collect()
}

pub fn to_char_vec<'a>(data: impl IntoIterator<Item = &'a str>) -> Vec<char> {
    data.into_iter()
        .map(|s| s.chars().next().unwrap())
        .collect()
}

pub fn to_char_vec2d<'a, const N: usize>(
    data: impl IntoIterator<Item = [&'a str; N]>,
) -> Vec<Vec<char>> {
    data.into_iter().map(|row| to_char_vec(row)).collect()
}

pub fn to_string_vec<S: AsRef<str>>(data: impl IntoIterator<Item = S>) -> Vec<String> {
    data.into_iter().map(|s| s.as_ref().to_string()).collect()
}

pub fn to_string_vec2d<S, II>(data: impl IntoIterator<Item = II>) -> Vec<Vec<String>>
where
    S: AsRef<str>,
    II: IntoIterator<Item = S>,
{
    data.into_iter()
        .map(|row| row.into_iter().map(|s| s.as_ref().to_string()).collect())
        .collect()
}
