
// Note that there is a problem with this function
// There is no link between the returned size and the
// original string.
// If it were modified later, the returned index could be invalid.
// pub fn first_word(s: &String) -> usize{
//     let bytes: &[u8] = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate(){
//         if item == b' '{
//             return i;
//         }
//     }
//     s.len()
// }

// Note the use of &str instead of &String
// A string slice is a reference to part of a String
// or a string literal
// we can now work on parts of a string or the enitre string
pub fn first_word(s: &str) -> &str{
    let bytes: &[u8] = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            // return &s[0..i]; equivalent to
            return &s[..i];
        }
    }
    // &s[0..s.len()]; equivalent to
    &s[..]
}