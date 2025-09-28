fn sort<T: Ord>(array: &mut [T]) {
    if array.len() <= 1 {
        return; // 递归基：长度≤1的数组已有序
    }
    
    // 选择基准值（中间元素避免最坏情况）
    let pivot_index = array.len() / 2;
    let last_index = array.len() - 1;
    array.swap(pivot_index, last_index); // 基准值移至末尾
    
    // Lomuto分区算法
    let mut i = 0;
    for j in 0..last_index {
        if array[j] <= array[last_index] {
            array.swap(i, j);
            i += 1;
        }
    }
    array.swap(i, last_index); // 基准值归位
    
    // 递归排序子数组（避免包含基准值）
    let (left, right) = array.split_at_mut(i);
    sort(left);
    sort(&mut right[1..]);
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