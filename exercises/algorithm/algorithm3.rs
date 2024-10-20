
fn sort<T: Ord + Copy>(array: &mut [T]) {
    quick_sort(array, 0, (array.len() - 1) as i32);
}


fn quick_sort<T: Ord + Copy>(array: &mut [T], low: i32, high: i32) {

    if low < high {
        let pi = partition(array, low, high);

        quick_sort(array, low, pi - 1);

        quick_sort(array, pi + 1, high);
    }
} 


fn partition<T: Ord + Copy>(array: &mut [T], low: i32, high: i32) -> i32 {
    let pivot = array[high as usize]; 
    let mut i = low - 1; 
    for j in low..high { 
        if array[j as usize] < pivot { 
            i += 1; 
            array.swap(i as usize, j as usize);
        }
    }

    array.swap((i + 1) as usize, high as usize);
    i + 1 
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