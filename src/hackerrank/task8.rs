// https://www.hackerrank.com/challenges/migratory-birds/problem

fn migratory_birds(arr: &[i32]) -> i32 {
    let mut counts = [0i32; 6];

    for &bird in arr {
        counts[bird as usize] += 1;
    }

    let mut best_type = 1;
    let mut best_count = 0;

    for bird_type in 1..=5 {
        if counts[bird_type as usize] > best_count {
            best_count = counts[bird_type as usize];
            best_type = bird_type;
        }
    }

    best_type
}

fn migratory_birds_print(arr: &[i32]) {
    println!("{}", migratory_birds(arr));
}

#[test]
fn test_migratory_birds_sample() {
    let arr = vec![1, 4, 4, 4, 5, 3];
    assert_eq!(migratory_birds(&arr), 4);
}

#[test]
fn test_migratory_birds_sample1() {
    let arr = vec![1, 2, 3, 4, 5, 4, 3, 2, 1, 3, 4];
    assert_eq!(migratory_birds(&arr), 3);
}

#[test]
fn test_migratory_birds_example() {
    let arr = vec![1, 1, 2, 2, 3];
    assert_eq!(migratory_birds(&arr), 1);
}

#[test]
fn test_migratory_birds_print() {
    migratory_birds_print(&[1, 4, 4, 4, 5, 3]);
}
