pub mod datastructures;
pub mod sorting;

#[cfg(test)]
mod tests {
    use super::sorting;
    use rand::Rng;

    fn gen_data(n: usize) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        for _ in 0..n {
            ans.push(rand::thread_rng().gen_range(1..10000));
        }
        ans
    }

    fn test_top<T>(
        arr: &mut [T],
        sorting_func: impl Fn(&mut [T]) -> (),
        fact_cmp: impl Fn(&T, &T) -> bool,
    ) where
        T: Ord + std::fmt::Debug,
    {
        sorting_func(arr);
        for i in 0..arr.len() - 1 {
            if fact_cmp(&arr[i], &arr[i + 1]) {
                panic!("Error in sorting...");
            }
        }
    }

    #[test]
    #[ignore]
    fn test_sorting() {
        let mut arr = gen_data(1000);
        println!("{:?}", arr);
        // test_top(&mut arr, sorting::bubble_sort, |&a, &b| a > b);
        // test_top(&mut arr, sorting::heap_sorting::heap_sort, |&a, &b| a > b);
        // test_top(&mut arr, sorting::insert_sort, |&a, &b| a > b);
        // test_top(&mut arr, sorting::merge_sorting::merge_sort, |&a, &b| a > b);
        test_top(&mut arr, sorting::quick_sorting::quick_sort, |&a, &b| a > b);
    }
}
