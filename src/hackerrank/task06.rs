// https://www.hackerrank.com/challenges/kangaroo/problem

fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> &'static str {
    if v1 == v2 {
        return if x1 == x2 { "YES" } else { "NO" };
    }
    if v1 <= v2 {
        return "NO";
    }
    if (x2 - x1) % (v1 - v2) == 0 {
        "YES"
    } else {
        "NO"
    }
}

fn kangaroo_print(x1: i32, v1: i32, x2: i32, v2: i32) {
    println!("{}", kangaroo(x1, v1, x2, v2));
}

#[test]
fn test_kangaroo_sample_yes() {
    assert_eq!(kangaroo(0, 3, 4, 2), "YES");
}

#[test]
fn test_kangaroo_sample_no() {
    assert_eq!(kangaroo(0, 2, 5, 3), "NO");
}

#[test]
fn test_kangaroo_same_speed() {
    assert_eq!(kangaroo(1, 2, 5, 2), "NO");
}

#[test]
fn test_kangaroo_print() {
    kangaroo_print(0, 3, 4, 2);
}
