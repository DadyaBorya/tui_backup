pub fn next(index: usize, len: usize) -> usize {
    (index + 1) % len
}

pub fn previous(index: usize, len: usize) -> usize {
    if index > 0 {
        index - 1
    } else {
       len - 1
    }
}
