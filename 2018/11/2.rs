const INPUT: i32 = 9221;
const SIZE: usize = 300;

fn main() {
    let mut fuel = Vec::new();
    let mut fuel_x = Vec::new();
    let mut fuel_y = vec![vec![0; SIZE]];
    for x in 1 ..= SIZE as i32 {
        let mut row = Vec::new();
        let mut row_x = vec![0];
        let mut row_y = Vec::new();
        for y in 1 ..= SIZE as i32 {
            let rack_id = x + 10;
            row.push(((rack_id * y + INPUT) * rack_id) / 100 % 10 - 5);
            let sum_x = row_x.last().unwrap() + row.last().unwrap();
            row_x.push(sum_x);
            let sum_y = fuel_y.last().unwrap()[(y - 1) as usize] + row.last().unwrap();
            row_y.push(sum_y);
        }
        fuel.push(row);
        fuel_x.push(row_x);
        fuel_y.push(row_y);
    }

    let mut best_x = 0;
    let mut best_y = 0;
    let mut best_s = 1;
    let mut best_sum = fuel[0][0];
    let mut sum_s = best_sum;
    for s in 1 ..= SIZE {
        if s > 0 {
            sum_s += fuel_x[s - 1][s];
            sum_s += fuel_y[s][s - 1];
            sum_s -= fuel[s - 1][s - 1];
        }
        let mut sum_x = sum_s;
        for x in 0 ..= SIZE - s {
            if x > 0 {
                sum_x -= fuel_x[x - 1][s];
                sum_x += fuel_x[x + s - 1][s];
            }
            let mut sum_y = sum_x;
            for y in 0 ..= SIZE - s {
                if y > 0 {
                    sum_y -= fuel_y[x + s][y - 1] - fuel_y[x][y - 1];
                    sum_y += fuel_y[x + s][y + s - 1] - fuel_y[x][y + s - 1];
                }
                if sum_y > best_sum {
                    best_x = x;
                    best_y = y;
                    best_s = s;
                    best_sum = sum_y;
                }
            }
        }
    }
    println!("{},{},{}", best_x + 1, best_y + 1, best_s);
}
