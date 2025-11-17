mod enums;
// RUST HAS NO NULL VALUES
// But it has the concept of non-existing values with the Option<T> enum

pub fn option_t() {
   let x:Option<i32> = Some(5);
   let a:Option<char> = Some('a');
   let nothin: Option<i32> = None;

}
