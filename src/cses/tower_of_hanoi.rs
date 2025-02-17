// The Tower of Hanoi game consists of three stacks (left, middle and right) and n round disks of different sizes. Initially, the left stack has all the disks, in increasing order of size from top to bottom.
// The goal is to move all the disks to the right stack using the middle stack. On each move you can move the uppermost disk from a stack to another stack. In addition, it is not allowed to place a larger disk on a smaller disk.
// Your task is to find a solution that minimizes the number of moves.
// Input
// The only input line has an integer n: the number of disks.
// Output
// First print an integer k: the minimum number of moves.
// After this, print k lines that describe the moves. Each line has two integers a and b: you move a disk from stack a to stack b.

pub fn tower_of_hanoi(_n: u32) -> Vec<(u32, u32)> {
    todo!()
}

// fn hanoi(n: u8, src: u8, tmp: u8, tgt: u8) {
//     if n == 0 { 
//         return; 
//     }
//     hanoi(n - 1, src, tgt, tmp);
//     println!("{} {}", src, tgt);
//     hanoi(n - 1, tmp, src, tgt);
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn tower_of_hanoi_test() {
//         assert_eq!(tower_of_hanoi(3), vec![(1, 2), (1, 3), (2, 3)]);
//     }
// }
