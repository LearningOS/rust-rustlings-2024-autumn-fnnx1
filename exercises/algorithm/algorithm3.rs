/*
    排序
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

// 冒泡排序
// fn sort<T: Ord>(array: &mut [T]){
// 	let n = array.len();

//     for i in 0..n {
//         for j in 0..(n - i - 1) {
//             if array[j] > array[j + 1] {
//                 array.swap(j, j + 1);
//             }
//         }
//     }
// }

fn sort<T: Ord + Copy>(array: &mut [T]) {
    quick_sort(array, 0, (array.len() - 1) as i32);
}

// 快速排序
fn quick_sort<T: Ord + Copy>(array: &mut [T], low: i32, high: i32) {
    // low < high 时，对内部元素排序
    if low < high {
        let pi = partition(array, low, high);
        // 递归左半部分
        quick_sort(array, low, pi - 1);
        // 递归右半部分
        quick_sort(array, pi + 1, high);
    }
} 

// 划分函数，找到基准元素的正确位置，排序完后会让该元素左侧都是比它小的，右侧都是比它大的
fn partition<T: Ord + Copy>(array: &mut [T], low: i32, high: i32) -> i32 {
    let pivot = array[high as usize]; // 选择最后一个元素作为基准
    let mut i = low - 1; // 初始化索引
    for j in low..high { // 遍历从 low 到 high - 1
        if array[j as usize] < pivot { // 如果该元素 < pivot
            i += 1; // 索引 + 1，让该元素移动到索引的位置
            array.swap(i as usize, j as usize);
        }
    }
    // 将基准元素放在正确位置
    array.swap((i + 1) as usize, high as usize);
    i + 1 // 返回基准元素的位置
}

fn main() {}

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