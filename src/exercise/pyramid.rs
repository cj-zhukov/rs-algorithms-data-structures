/// Printing Pyramid Patterns in Rust
/// for n = 10, result:
///          *
///         ***
///        *****
///       *******
///      *********
///     ***********
///    *************
///   ***************
///  *****************
/// *******************
pub fn print_pyramid(n: usize) {
    for i in 0..n {
        print!("{}", " ".repeat(n - i - 1));
        println!("{}", "*".repeat(2 * i + 1));
    }
}
