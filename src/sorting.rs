#![allow(dead_code)]

pub fn bubble_sort<T>(arr: &mut [T])
where
    T: Ord,
{
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

pub mod heap_sorting {
    fn build_heap<T>(arr: &mut [T])
    where
        T: std::fmt::Debug + Ord,
    {
        let last_parent = (arr.len() - 2) << 1;
        for i in (0..=last_parent).rev() {
            shift_down(arr, i);
        }
    }

    fn shift_down<T>(arr: &mut [T], pos: usize)
    where
        T: std::fmt::Debug + Ord,
    {
        let last = arr.len() - 1;
        let mut root = pos;
        let mut son = (root << 1) + 1;
        while son <= last {
            if son + 1 <= last && arr[son] < arr[son + 1] {
                son = son + 1;
            }
            if arr[son] < arr[root] {
                break;
            }
            arr.swap(root, son);
            root = son;
            son = (root << 1) + 1;
        }
    }

    pub fn heap_sort<T>(arr: &mut [T])
    where
        T: std::fmt::Debug + Ord,
    {
        build_heap(arr);
        for end in (1..arr.len()).rev() {
            arr.swap(0, end);
            shift_down(&mut arr[..end], 0);
        }
    }
}

pub fn insert_sort<T>(arr: &mut [T])
where
    T: std::fmt::Debug + Ord + Copy,
{
    let len = arr.len();
    for i in 1..len {
        let mut j = i - 1;
        let val = arr[i];
        while arr[j] > val {
            arr.swap(j + 1, j);
            if j == 0 {
                break;
            }
            j -= 1;
        }
    }
}

pub mod merge_sorting {
    pub fn merge_sort<T>(arr: &mut [T])
    where
        T: std::fmt::Debug + Ord + Copy,
    {
        split(arr, 0, arr.len());
    }

    /// [start, end] minus overflow
    /// [start, end)
    fn split<T>(arr: &mut [T], start: usize, end: usize)
    where
        T: std::fmt::Debug + Ord + Copy,
    {
        if start + 1 >= end {
            // Attention ! ! !
            return;
        }
        let mid = (end - start) / 2 + start;
        split(arr, start, mid);
        split(arr, mid, end);
        merge(arr, start, mid, mid, end);
    }

    fn merge<T>(arr: &mut [T], s1: usize, e1: usize, s2: usize, e2: usize)
    where
        T: std::fmt::Debug + Ord + Copy,
    {
        let mut i = s1;
        let mut j = s2;
        let vec = arr.to_vec();
        let mut top = s1;
        while i < e1 && j < e2 {
            if vec[i] < vec[j] {
                arr[top] = vec[i];
                i += 1;
            } else {
                arr[top] = vec[j];
                j += 1;
            }
            top += 1;
        }
        while i < e1 {
            arr[top] = vec[i];
            i += 1;
            top += 1;
        }

        while j < e2 {
            arr[top] = vec[j];
            j += 1;
            top += 1;
        }
    }
}

pub mod quick_sorting {
    pub fn quick_sort<T>(arr: &mut [T])
    where
        T: std::fmt::Debug + Copy + Ord,
    {
        _quick_sort(arr, 0, arr.len());
        println!("{:?}", arr);
    }

    /// [start, end)
    fn _quick_sort<T>(arr: &mut [T], start: usize, end: usize)
    where
        T: std::fmt::Debug + Copy + Ord,
    {
        if start + 1 >= end {
            return;
        }
        let mid = partition(arr, start, end);
        _quick_sort(arr, start, mid);
        _quick_sort(arr, mid + 1, end);
    }

    fn partition<T>(arr: &mut [T], start: usize, end: usize) -> usize
    where
        T: std::fmt::Debug + Copy + Ord,
    {
        let x = arr[start];
        let mut i = start as isize;
        let mut j = (end - 1) as isize;

        while i < j {
            while i < j && arr[j as usize] >= x {
                j -= 1;
            }
            if i < j {
                arr[i as usize] = arr[j as usize];
                i += 1;
            }
            while i < j && arr[i as usize] < x {
                i += 1;
            }
            if i < j {
                arr[j as usize] = arr[i as usize];
                j -= 1;
            }
        }
        arr[i as usize] = x;
        i as usize
    }
}
