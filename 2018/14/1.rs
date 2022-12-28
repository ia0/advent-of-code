fn main() {
    const INPUT: usize = 440231;
    let mut recipes: Vec<u8> = vec![3, 7];
    let mut x = 0;
    let mut y = 1;
    while recipes.len() < INPUT + 10 {
        let mut z = recipes[x] + recipes[y];
        if z > 9 {
            assert!(z < 20);
            z -= 10;
            recipes.push(1);
        }
        recipes.push(z);
        x = (x + 1 + recipes[x] as usize) % recipes.len();
        y = (y + 1 + recipes[y] as usize) % recipes.len();
    }
    for x in &recipes[INPUT .. INPUT + 10] {
        print!("{}", x);
    }
    println!();
}
