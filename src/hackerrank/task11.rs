// https://www.hackerrank.com/challenges/drawing-book/problem

fn page_count(n: i32, p: i32) -> i32 {
    let from_front = p / 2;
    let from_back = n / 2 - p / 2;

    from_front.min(from_back)
}

fn page_count_print(n: i32, p: i32) {
    println!("{}", page_count(n, p));
}

#[test]
fn test_page_count_sample0() {
    assert_eq!(page_count(6, 2), 1);
}

#[test]
fn test_page_count_sample1() {
    assert_eq!(page_count(5, 4), 0);
}

#[test]
fn test_page_count_example_n5_p3() {
    assert_eq!(page_count(5, 3), 1);
}

#[test]
fn test_page_count_first_page() {
    assert_eq!(page_count(10, 1), 0);
}

#[test]
fn test_page_count_last_page_odd() {
    assert_eq!(page_count(7, 7), 0);
}

#[test]
fn test_page_count_last_page_even() {
    assert_eq!(page_count(6, 6), 0);
}

#[test]
fn test_page_count_single_page() {
    assert_eq!(page_count(1, 1), 0);
}

#[test]
fn test_page_count_middle_large() {
    assert_eq!(page_count(100_000, 50_000), 25_000);
}

#[test]
fn test_page_count_print() {
    page_count_print(6, 2);
}
