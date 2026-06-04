// https://www.hackerrank.com/challenges/breaking-best-and-worst-records/problem

fn breaking_records(scores: &[i32]) -> (i32, i32) {
    let mut max_score = scores[0];
    let mut min_score = scores[0];
    let mut max_count = 0;
    let mut min_count = 0;

    for &score in &scores[1..] {
        if score > max_score {
            max_score = score;
            max_count += 1;
        } else if score < min_score {
            min_score = score;
            min_count += 1;
        }
    }

    (max_count, min_count)
}

fn breaking_records_print(scores: &[i32]) {
    let (max_count, min_count) = breaking_records(scores);
    println!("{} {}", max_count, min_count);
}

#[test]
fn test_breaking_records_sample() {
    let scores = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];
    assert_eq!(breaking_records(&scores), (2, 4));
}

#[test]
fn test_breaking_records_example() {
    let scores = vec![12, 24, 10, 24];
    assert_eq!(breaking_records(&scores), (1, 1));
}

#[test]
fn test_breaking_records_sample1() {
    let scores = vec![3, 4, 21, 36, 10, 28, 35, 5, 24, 42];
    assert_eq!(breaking_records(&scores), (4, 0));
}

#[test]
fn test_breaking_records_print() {
    breaking_records_print(&[12, 24, 10, 24]);
}
