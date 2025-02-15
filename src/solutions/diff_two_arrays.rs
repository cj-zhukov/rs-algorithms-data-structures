// Compare two arrays and return a new array with any items only found in one of the two given arrays,
// but not both. In other words, return the symmetric difference of the two arrays.

// Note: You can return the array with its elements in any order.

pub fn diff_array<'a>(mut arr1: Vec<&'a str>, arr2: Vec<&'a str>) -> Vec<&'a str> {
    // Check if arr1 contains strings of arr2,
    // if it does delete the string, else push it
    for el in arr2 {
        if arr1.contains(&el) {
            let index = arr1.iter().position(|&x| x == el).unwrap();
            arr1.remove(index);
        } else {
            arr1.push(el);
        }
    }
    arr1.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    arr1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let arr1 = vec![
            "diorite",
            "andesite",
            "grass",
            "dirt",
            "pink wool",
            "dead shrub",
        ];
        let arr2 = vec!["diorite", "andesite", "grass", "dirt", "dead shrub"];
        assert_eq!(diff_array(arr1, arr2), ["pink wool"]);
    }

    #[test]
    fn test2() {
        let arr1 = vec!["andesite", "grass", "dirt", "pink wool", "dead shrub"];
        let arr2 = vec!["diorite", "andesite", "grass", "dirt", "dead shrub"];
        let mut answer = diff_array(arr1, arr2);
        answer.sort_unstable();
        assert_eq!(answer, ["diorite", "pink wool"]);
    }

    #[test]
    fn test3() {
        let arr1 = vec!["andesite", "grass", "dirt", "dead shrub"];
        let arr2 = vec!["andesite", "grass", "dirt", "dead shrub"];
        assert_eq!(diff_array(arr1, arr2).len(), 0);
    }
}
