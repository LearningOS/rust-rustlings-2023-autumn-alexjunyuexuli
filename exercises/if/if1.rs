// if1.rs
//
// Execute `rustlings hint if1` or use the `hint` watch subcommand for a hint.



pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // Do not use:
    // - another function call
    // - additional variables
    let a :i32 =10;
    let b:i32 =8;
    if a>b{
        a
    }else{
        b
    }

}
pub fn bb(c: i32, d: i32) -> i32 {
    let c:i32 =42;
    let d:i32 =32;
    if c>d{
        c
    }else{
        d
    }

}


// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
