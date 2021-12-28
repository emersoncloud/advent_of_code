fn test_board_solitions() {
    let mut x: Vec<Vec<u32>> = vec![
        vec![1, 2, 3, 4, 5],
        vec![1, 2, 3, 4, 5],
        vec![1, 2, 3, 4, 5],
        vec![1, 2, 3, 4, 5],
        vec![1, 2, 3, 4, 5],
    ];

    let mut possible_wins: Vec<HashSet<u32>> = vec![];
    for i in &x {
        let mut hash: HashSet<u32> = HashSet::new();
        for j in i {
            hash.insert(*j);
        }
        possible_wins.push(hash);
    }


    let mut j = 0;
    while &j < &x.len() {
        let mut hash: HashSet<u32> = HashSet::new();
        for i in 0..x.len() {
            let val = x[i][j];
            hash.insert(x[i][j]);
        }
        possible_wins.push(hash);
        j += 1;
    }

    println!("debug");
}
