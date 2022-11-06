use std::collections::HashSet;

pub fn func(s: String) -> i32 {
    let mut chars_occurence: HashSet<&u8> = HashSet::new();

    let bytes = s.into_bytes();
    let mut iter = bytes.iter();
    let mut max_lenght = 0;
    while let Some(val) = iter.next()  {
        if !chars_occurence.insert(val) {
            let current_lenght = chars_occurence.len();
            if current_lenght > max_lenght {
                max_lenght = current_lenght;
            }
            chars_occurence.clear();
            chars_occurence.insert(val);
        }
    }

    max_lenght as i32
}
