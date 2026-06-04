// https://www.hackerrank.com/challenges/sock-merchant/problem

fn sock_merchant(ar: &[i32]) -> i32 {
    let mut counts = [0i32; 101];

    for &color in ar {
        counts[color as usize] += 1;
    }

    counts.iter().map(|&count| count / 2).sum()
}

fn sock_merchant_print(ar: &[i32]) {
    println!("{}", sock_merchant(ar));
}

#[test]
fn test_sock_merchant_sample() {
    let ar = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];
    assert_eq!(sock_merchant(&ar), 3);
}

#[test]
fn test_sock_merchant_example() {
    let ar = vec![1, 2, 1, 2, 1, 3, 2];
    assert_eq!(sock_merchant(&ar), 2);
}

#[test]
fn test_sock_merchant_no_pairs() {
    let ar = vec![1, 2, 3];
    assert_eq!(sock_merchant(&ar), 0);
}

#[test]
fn test_sock_merchant_print() {
    sock_merchant_print(&[10, 20, 20, 10, 10, 30, 50, 10, 20]);
}
