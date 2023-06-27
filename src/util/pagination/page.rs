pub struct Page<'a, T> {
    entries: &'a [T],
    has_next_page: bool,
}
