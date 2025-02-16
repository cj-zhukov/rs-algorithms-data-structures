// Flatten a nested array. You must account for varying levels of nesting.

pub enum Value {
    Number(i32),
    Array(Vec<Value>),
}

pub fn steamroll_array(arr: &[Value]) -> Vec<i32> {
    let mut res = vec![];
    for val in arr {
        match val {
            Value::Number(x) => res.push(*x),
            Value::Array(xs) => res.extend(steamroll_array(xs)),
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn streamroll_array_test() {
        let arr = [Value::Number(1), Value::Array(vec![Value::Number(2)])];
        let res = steamroll_array(&arr);
        assert_eq!(res, vec![1, 2]);
    }
}
