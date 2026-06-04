// https://www.hackerrank.com/challenges/grading/problem

fn round_grade(grade: i32) -> i32 {
    if grade < 38 {
        return grade;
    }
    let remainder = grade % 5;
    if remainder == 0 {
        return grade;
    }
    let diff = 5 - remainder;
    if diff < 3 {
        grade + diff
    } else {
        grade
    }
}

fn grading_students(grades: &[i32]) -> Vec<i32> {
    grades.iter().map(|&grade| round_grade(grade)).collect()
}

#[test]
fn test_grading_sample() {
    let grades = vec![73, 67, 38, 33];
    let expected = vec![75, 67, 40, 33];
    assert_eq!(grading_students(&grades), expected);
}

#[test]
fn test_grading_examples() {
    assert_eq!(round_grade(84), 85);
    assert_eq!(round_grade(29), 29);
    assert_eq!(round_grade(57), 57);
}

#[test]
fn test_grading_no_change() {
    assert_eq!(round_grade(40), 40);
    assert_eq!(round_grade(100), 100);
}
