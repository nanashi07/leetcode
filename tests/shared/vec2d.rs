pub fn to_vec2d<T: Clone, const N: usize>(data: impl IntoIterator<Item = [T; N]>) -> Vec<Vec<T>> {
    data.into_iter().map(|row| row.to_vec()).collect()
}

