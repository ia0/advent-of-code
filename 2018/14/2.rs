fn main() {
    const INPUT: &[u8] = &[4, 4, 0, 2, 3, 1];
    let mut recipes: Vec<u8> = vec![3, 7];
    let mut x = 0;
    let mut y = 1;
    let push = |r: &mut Vec<u8>, x| {
        r.push(x);
        let n = r.len();
        return n >= INPUT.len() && &r[n - INPUT.len() ..] == INPUT;
    };
    loop {
        let mut z = recipes[x] + recipes[y];
        if z > 9 {
            assert!(z < 20);
            z -= 10;
            if push(&mut recipes, 1) {
                break;
            }
        }
        if push(&mut recipes, z) {
            break;
        }
        x = (x + 1 + recipes[x] as usize) % recipes.len();
        y = (y + 1 + recipes[y] as usize) % recipes.len();
    }
    println!("{}", recipes.len() - INPUT.len());
}
