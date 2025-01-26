pub fn sort_algo_1<T: PartialOrd>(arr: &mut Vec<T>) {
    let mut swap= false;
    for i in 0..(arr.len()-1) {
        if arr[i] > arr[i+1] {
            arr.swap(i, i+1);
            swap = true;
        }
        if swap {
            sort_algo_1(arr);
        }
    }
}

pub fn sort_algo_2<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for left in 0..len {
        let mut smallest = left;
        for right in (left + 1)..len {
            if arr[right] < arr[smallest] {
                smallest = right;
            }
        }
        arr.swap(smallest, left);
    }
}