pub fn sort<T>(slice: &mut [T])
where T: Ord 
{
    let mut swapped = true;
    while swapped {
        swapped = false;        
        for i in 0..(slice.len() - 1) {
            if slice[i] > slice[i + 1] {
                slice.swap(i, i + 1);
                swapped = true;
            }
        }
    }
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn sort_test() {
        let mut things = vec![4, 2, 3, 1];
        sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4]);
    }
}
