/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: PartialOrd + Copy>(array: &mut [T]) {
    //TODO
    let len = array.len();
    if len < 2 {
        return;
    }
    let mid = len / 2;
    sort(&mut array[..mid]);
    sort(&mut array[mid..]);
    let left = &array[..mid];
    let right = &array[mid..];
    let mut v: Vec<T> = vec![array[0]; len];
    let (mut i, mut j) = (0, 0);
    while i != mid && j != len - mid {
        if left[i] < right[j] {
            v[i + j] = left[i];
            i += 1;
        } else {
            v[i + j] = right[j];
            j += 1;
        }
    }

    while i != mid {
        v[i + j] = left[i];
        i += 1;
    }

    while j != len - mid {
        v[i + j] = right[j];
        j += 1;
    }

    (0..len).for_each(|i| array[i] = v[i]);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
