

struct Test {
    score: i32,
}

pub fn vectors() {
    let scores: Vec<Test> = vec![
        Test{score: 1},
        Test{score: 2},
        Test{score: 3},
    ];
    
    for sc in scores {
        println!("score: {:?}", sc.score)
    };
    return
}
