// https://www.hackerrank.com/challenges/staircase/problem

fn staircase_output(n: i32) -> String {
    (1..=n)
        .map(|i| {
            format!(
                "{}{}",
                " ".repeat((n - i) as usize),
                "#".repeat(i as usize)
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn staircase(n: i32) {
    println!("{}", staircase_output(n));
}

#[test]
fn test_staircase() {
    let expected = "   #\n  ##\n ###\n####";
    assert_eq!(staircase_output(4), expected);
}

#[test]
fn test_staircase_sample() {
    let expected = "     #\n    ##\n   ###\n  ####\n #####\n######";
    assert_eq!(staircase_output(6), expected);
}

#[test]
fn test_staircase_single_step() {
    assert_eq!(staircase_output(1), "#");
    staircase(1);
}
