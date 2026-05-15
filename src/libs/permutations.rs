pub fn prev_permutation(arr: &mut [usize]) -> bool {
    let n = arr.len();
    let mut i = n - 1;
    while i > 0 && arr[i - 1] <= arr[i] {
        i -= 1;
    }
    if i == 0 {
        return false;
    }
    let mut j = n - 1;
    while arr[j] >= arr[i - 1] {
        j -= 1;
    }
    arr.swap(i - 1, j);
    arr[i..].reverse();
    true
}
