// https://www.hackerrank.com/challenges/diagonal-difference/problem

fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut primary = 0;
    let mut secondary = 0;

    for i in 0..n {
        primary += arr[i][i];
        secondary += arr[i][n - 1 - i];
    }

    (primary - secondary).abs()
}

fn diagonal_difference_print(arr: &[Vec<i32>]) {
    println!("{}", diagonal_difference(arr));
}

#[test]
fn test_diagonal_difference_sample() {
    let arr = vec![vec![11, 2, 4], vec![4, 5, 6], vec![10, 8, -12]];
    assert_eq!(diagonal_difference(&arr), 15);
}

#[test]
fn test_diagonal_difference_example() {
    let arr = vec![vec![1, 2, 3], vec![4, 5, 6], vec![9, 8, 9]];
    assert_eq!(diagonal_difference(&arr), 2);
}

#[test]
fn test_diagonal_difference_single() {
    let arr = vec![vec![5]];
    assert_eq!(diagonal_difference(&arr), 0);
}

#[test]
fn test_diagonal_difference_equal_diagonals() {
    let arr = vec![vec![1, 2], vec![3, 4]];
    assert_eq!(diagonal_difference(&arr), 0);
}

#[test]
fn test_diagonal_difference_print() {
    diagonal_difference_print(&[vec![11, 2, 4], vec![4, 5, 6], vec![10, 8, -12]]);
}
