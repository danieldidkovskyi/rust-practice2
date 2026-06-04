// https://www.hackerrank.com/challenges/apple-and-orange/problem

fn count_fruits(s: i32, t: i32, tree: i32, distances: &[i32]) -> i32 {
    distances
        .iter()
        .filter(|&&d| {
            let pos = tree + d;
            pos >= s && pos <= t
        })
        .count() as i32
}

fn count_apples_and_oranges(
    s: i32,
    t: i32,
    a: i32,
    b: i32,
    apples: &[i32],
    oranges: &[i32],
) -> (i32, i32) {
    (
        count_fruits(s, t, a, apples),
        count_fruits(s, t, b, oranges),
    )
}

fn count_apples_and_oranges_print(
    s: i32,
    t: i32,
    a: i32,
    b: i32,
    apples: &[i32],
    oranges: &[i32],
) {
    let (apple_count, orange_count) =
        count_apples_and_oranges(s, t, a, b, apples, oranges);
    println!("{}\n{}", apple_count, orange_count);
}

#[test]
fn test_apple_and_orange_sample() {
    let apples = vec![-2, 2, 1];
    let oranges = vec![5, -6];
    let result = count_apples_and_oranges(7, 11, 5, 15, &apples, &oranges);
    assert_eq!(result, (1, 1));
}

#[test]
fn test_apple_and_orange_example() {
    let apples = vec![2, 3, -4];
    let oranges = vec![3, -2, -4];
    let result = count_apples_and_oranges(7, 10, 4, 12, &apples, &oranges);
    assert_eq!(result, (1, 2));
}

#[test]
fn test_apple_and_orange_print() {
    count_apples_and_oranges_print(7, 11, 5, 15, &[-2, 2, 1], &[5, -6]);
}
