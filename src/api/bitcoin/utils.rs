// LastIndexByte returns the index of the last instance of c in s, or -1 if c is not present in s.
pub fn last_index_byte(s: &str, c: u8) -> isize {
    for i in (0..s.len()).rev() {
        println!("{}", i);

        if s.as_bytes()[i] == c {
            return i.try_into().unwrap();
        }
    }

    return -1;
}
