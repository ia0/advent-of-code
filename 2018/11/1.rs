const INPUT: i32 = 9221;

fn get_fuel(x: i32, y: i32) -> i32 {
    let rack_id = x + 10;
    let mut fuel = rack_id * y;
    fuel += INPUT;
    fuel *= rack_id;
    fuel /= 100;
    fuel %= 10;
    fuel - 5
}

fn get_sum(fuel: &[Vec<i32>], x: i32, y: i32) -> i32 {
    let mut sum = 0;
    for x in x - 1 .. x + 2 {
        for y in y - 1 .. y + 2 {
            sum += fuel[x as usize][y as usize];
        }
    }
    sum
}

fn main() {
    let mut fuel = Vec::new();
    for x in 1 ..= 300 {
        let mut row = Vec::new();
        for y in 1 ..= 300 {
            row.push(get_fuel(x, y));
        }
        fuel.push(row);
    }
    let mut best_x = 1;
    let mut best_y = 1;
    let mut best_fuel = 0;
    for x in 1 .. 299 {
        for y in 1 .. 299 {
            let sum = get_sum(&fuel, x, y);
            if (x == 1 && y == 1) || sum > best_fuel {
                best_x = x;
                best_y = y;
                best_fuel = sum;
            }
        }
    }
    println!("{},{}", best_x, best_y);
}
