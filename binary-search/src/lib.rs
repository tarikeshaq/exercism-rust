pub fn find<G, T>(array: G, key: T) -> Option<usize>
where
    T: PartialOrd + Copy,
    G: AsRef<[T]>,
{
    let (mut start, mut end) = (0 as i32, array.as_ref().len() as i32 - 1);
    while start <= end {
        let mid = start + (end - start) / 2;
        let curr_val = *array.as_ref().get(mid as usize).unwrap();
        if curr_val == key {
            return Some(mid as usize);
        }
        if curr_val < key {
            start = mid + 1;
        } else {
            end = mid - 1;
        }
    }
    None
}
