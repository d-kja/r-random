fn main() {
    num_vec();
    score_vec();
}

fn score_vec() {
    struct Test {
        grade: f32,
    }

    let tests: Vec<Test> = vec![
        Test { grade: 7.5 },
        Test { grade: 5.5 },
        Test { grade: 9.5 },
        Test { grade: 7.5 },
    ];

    for test in tests {
        println!("grade: {}", test.grade);
    }
}

fn num_vec() {
    let mut n: Vec<u32> = Vec::new();

    n.push(1);
    n.push(2);
    n.push(3);
    n.pop();

    dbg!(&n, assert_eq!(n, [1, 2]));
}
