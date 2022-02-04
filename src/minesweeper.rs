pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut minefield_mat: Vec<Vec<u8>> = Vec::new();

    for mine_row in minefield {
        let mines = mine_row.as_bytes();
        minefield_mat.push(mines.to_vec());
    }

    let mut mine_field = minefield_mat.clone();

    for (j, y) in minefield_mat.iter().enumerate() {
        for (i, x) in y.iter().enumerate() {
            if !contains_asterix(x) {
                let total_count = count_horizontal(&mine_field, i, j);
                mine_field[j][i] = get_desired_char_for_mine(total_count);
            }
        }
    }

    mine_field
        .iter()
        .map(|row| {
            let mut stringified_mine_row = String::new();
            row.iter()
                .for_each(|byte| stringified_mine_row.push(*byte as char));
            stringified_mine_row
        })
        .collect()
}

fn count_horizontal(mine_field: &Vec<Vec<u8>>, x: usize, y: usize) -> u8 {
    let horizontal_possibilities: [(i8, i8); 4] = [(1, 1), (-1, -1), (-1, 1), (1, -1)];
    let mut asterixes: u8 = 0;

    for possibility in horizontal_possibilities {
        let (y, x) = match get_coordinates(&possibility, &x, &y) {
            Ok((y, x)) => (y, x),
            Err(_) => continue,
        };

        if is_pos_available(mine_field, y) && is_pos_available(&mine_field[y], x) {
            let location = mine_field[y][x];
            if contains_asterix(&location) {
                asterixes += 1;
            }
        }
    }

    return asterixes;
}

fn count_vertical(mine_field: &Vec<Vec<u8>>, x: usize, y: usize) -> u8 {
    let vertical_possibilities: [i8; 2] = [-1, 1];
    let mut asterixes: u8 = 0;

    for possibility in vertical_possibilities {
        let (y, x) = match get_coordinates(&(y as i8 + possibility, x as i8), &x, &y) {
            Ok((y, x)) => (y, x),
            Err(_) => continue,
        };

        if is_pos_available(mine_field, y) && is_pos_available(&mine_field[y], x) {
            let location = mine_field[y][x];
            if contains_asterix(&location) {
                asterixes += 1;
            }
        }
    }

    return asterixes;
}

fn contains_asterix(byte: &u8) -> bool {
    *byte as char == '*'
}

fn is_pos_available<T>(slice: &[T], index: usize) -> bool {
    slice.len() > index
}

fn get_desired_char_for_mine(asterixes: u8) -> u8 {
    if asterixes > 0 {
        asterixes
    } else {
        String::from(" ").as_bytes()[0]
    }
}

fn get_coordinates(
    possibility: &(i8, i8),
    x: &usize,
    y: &usize,
) -> Result<(usize, usize), &'static str> {
    let y = match (*y as i8 + possibility.0).try_into() {
        Ok(i) => i,
        Err(_) => return Err("Couldn't convert the y value"),
    };
    let x = match (*x as i8 + possibility.1).try_into() {
        Ok(i) => i,
        Err(_) => return Err("Couldn't convert the x value"),
    };

    Ok((y, x))
}
