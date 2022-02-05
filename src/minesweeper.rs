pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut minefield_mat: Vec<Vec<&str>> = Vec::new();

    for mine_row in minefield {
        let mines: Vec<&str> = mine_row.split("").collect();
        minefield_mat.push(mines[1..mines.len() - 1].to_vec());
    }

    let mut mine_field = minefield_mat.clone();

    for (j, y) in minefield_mat.iter().enumerate() {
        for (i, x) in y.iter().enumerate() {
            if !contains_asterix(*x) {
                let total_count = count_diagnol(&mine_field, i, j)
                    + count_vertical(&mine_field, i, j)
                    + count_horizontal(&mine_field, i, j);
                let desired_character = get_desired_char_for_mine(&total_count);
                if total_count > 0 {
                    mine_field[j][i] = Box::leak(desired_character.into_boxed_str());
                }
            }
        }
    }

    mine_field.iter().map(|row| row.join("")).collect()
}

fn count_diagnol(mine_field: &Vec<Vec<&str>>, x: usize, y: usize) -> u8 {
    let diagnol_possibilities: [(i8, i8); 4] = [(1, 1), (-1, -1), (-1, 1), (1, -1)];
    let mut asterixes: u8 = 0;

    for possibility in diagnol_possibilities {
        let (y, x) = match get_coordinates(&possibility, &x, &y) {
            Ok((y, x)) => (y, x),
            Err(_) => continue,
        };

        count_asterixes(mine_field, x, y, &mut asterixes);
    }

    return asterixes;
}

fn count_vertical(mine_field: &Vec<Vec<&str>>, x: usize, y: usize) -> u8 {
    let vertical_possibilities: [(i8, i8); 2] = [(0, -1), (0, 1)];
    let mut asterixes: u8 = 0;

    for possibility in vertical_possibilities {
        let (y, x) = match get_coordinates(&possibility, &x, &y) {
            Ok((y, x)) => (y, x),
            Err(_) => continue,
        };

        count_asterixes(mine_field, x, y, &mut asterixes);
    }

    return asterixes;
}

fn count_horizontal(mine_field: &Vec<Vec<&str>>, x: usize, y: usize) -> u8 {
    let horizontal_possibilities: [(i8, i8); 2] = [(-1, 0), (1, 0)];
    let mut asterixes: u8 = 0;

    for possibility in horizontal_possibilities {
        let (y, x) = match get_coordinates(&possibility, &x, &y) {
            Ok((y, x)) => (y, x),
            Err(_) => continue,
        };

        count_asterixes(mine_field, x, y, &mut asterixes);
    }

    return asterixes;
}

fn contains_asterix(character: &str) -> bool {
    character == "*"
}

fn is_pos_available<T>(slice: &[T], index: usize) -> bool {
    slice.len() > index
}

fn get_desired_char_for_mine(asterixes_count: &u8) -> String {
    if *asterixes_count > 0 {
        format!("{}", &asterixes_count)
    } else {
        "  ".to_string()
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

fn count_asterixes(mine_field: &Vec<Vec<&str>>, x: usize, y: usize, asterixes: &mut u8) {
    if is_pos_available(mine_field, y) && is_pos_available(&mine_field[y], x) {
        let location = mine_field[y][x];
        if contains_asterix(&location) {
            *asterixes += 1;
        }
    }
}
