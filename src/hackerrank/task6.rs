// https://www.hackerrank.com/challenges/between-two-sets/problem

fn gcd(a: i32, b: i32) -> i32 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: i32, b: i32) -> i32 {
    a / gcd(a, b) * b
}

fn lcm_all(values: &[i32]) -> i32 {
    values.iter().copied().reduce(lcm).unwrap()
}

fn gcd_all(values: &[i32]) -> i32 {
    values.iter().copied().reduce(gcd).unwrap()
}

fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let lcm_a = lcm_all(a);
    let gcd_b = gcd_all(b);
    let mut count = 0;
    let mut x = lcm_a;

    while x <= gcd_b {
        if gcd_b % x == 0 {
            count += 1;
        }
        x += lcm_a;
    }

    count
}

#[test]
fn test_between_two_sets_sample() {
    let a = vec![2, 4];
    let b = vec![16, 32, 96];
    assert_eq!(get_total_x(&a, &b), 3);
}

#[test]
fn test_between_two_sets_example() {
    let a = vec![2, 6];
    let b = vec![24, 36];
    assert_eq!(get_total_x(&a, &b), 2);
}

#[test]
fn test_between_two_sets_simple() {
    let a = vec![2];
    let b = vec![4];
    assert_eq!(get_total_x(&a, &b), 2);
}
