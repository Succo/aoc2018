fn main() -> std::io::Result<()> {
    let input = 9110;
    println!("{}", input);

    let mut grid = vec![vec![0; 300]; 300];

    for x in 1..301 {
        for y in 1..301 {
            let rack_id = x as isize + 10;
            let mut p_lvl: isize = rack_id * y as isize;
            p_lvl += input;
            p_lvl = p_lvl * rack_id;
            p_lvl = p_lvl / 100;
            p_lvl = p_lvl - ((p_lvl / 10) * 10);
            grid[x-1][y-1] = p_lvl - 5;
        }
    }

    let mut max = -6;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_size = 0;
    for size in 1..301 {
        for x in 1..301-size {
            for y in 1..301-size {
                let mut val = 0;
                for i in 0..size {
                    for j in 0..size {
                        val += grid[x+i][y+j]
                    }
                }
                if val > max {
                    max = val;
                    max_x = x+1;
                    max_y = y+1;
                    max_size = size;
                }
            }
        }
        println!("{}", size);
    }

    println!("{},{},{}", max_x, max_y, max_size);
    Ok(())
}

