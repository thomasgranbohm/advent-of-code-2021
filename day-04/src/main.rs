fn main() {
    let lines: Vec<String> = include_str!("../input.txt")
        .lines()
        .map(|value| value.parse().unwrap())
        .collect();

    part1(&lines);
    part2(&lines);
}

struct BingoCard {
    board: [[i8; 5]; 5],
    marked: [[bool; 5]; 5],
}

fn numbers_from_line(line: &String, delimiter: char) -> Vec<i8> {
    let mut numbers: Vec<i8> = Vec::new();
    let mut comp = String::new();

    for c in line.chars() {
        if c != delimiter {
            comp.push_str(&String::from(c));
        } else if comp.len() > 0 {
            numbers.push(comp.parse::<i8>().unwrap());
            comp = String::new();
        }
    }

    return numbers;
}

fn parse_line(line: String) -> [i8; 5] {
    let mut numbers: [i8; 5] = [-1; 5];
    let mut index = 0;
    let mut comp = String::new();

    for c in line.chars() {
        if c != ' ' {
            comp.push_str(&String::from(c));
        } else if comp.len() > 0 {
            numbers[index] = comp.parse::<i8>().unwrap();
            index += 1;
            comp = String::new();
        }
    }
    numbers[index] = comp.parse::<i8>().unwrap();

    return numbers;
}

fn parse_board(mut lines: Vec<String>) -> BingoCard {
    let mut board: [[i8; 5]; 5] = [[-1; 5]; 5];
    let marked: [[bool; 5]; 5] = [[false; 5]; 5];

    for y in 0..board.len() {
        let line = lines.pop().unwrap();
        board[y] = parse_line(line);
    }

    let bingo = BingoCard {
        board: board,
        marked: marked,
    };

    return bingo;
}

fn update_board(bingo: &BingoCard, number: i8) -> BingoCard {
    let mut clone = BingoCard {
        board: bingo.board,
        marked: bingo.marked,
    };
    for y in 0..clone.board.len() {
        let row = clone.board[y];

        for x in 0..clone.board.len() {
            let col = row[x];

            if col == number {
                clone.marked[y][x] = true;
            }
        }
    }

    return clone;
}

fn check_board(bingo: &BingoCard) -> i32 {
    let mut is_bingo = false;

    // Horizontal
    for y in 0..5 {
        let mut hor_win = true;
        for x in 0..5 {
            if !bingo.marked[y][x] {
                hor_win = false;
                break;
            }
        }
        if hor_win {
            is_bingo = hor_win;
            break;
        }
    }

    // Vertical
    if !is_bingo {
        for x in 0..5 {
            let mut ver_win = true;
            for y in 0..5 {
                if !bingo.marked[y][x] {
                    ver_win = false;
                    break;
                }
            }

            if ver_win {
                is_bingo = ver_win;
                break;
            }
        }
    }

    if is_bingo {
        let mut sum: i32 = 0;
        for y in 0..5 {
            for x in 0..5 {
                if !bingo.marked[y][x] {
                    sum += i32::from(bingo.board[y][x]);
                }
            }
        }
        return sum;
    }

    return -1;
}

fn part1(lines: &Vec<String>) {
    let mut cloned = lines.clone();
    cloned.reverse();
    let input = cloned.pop().unwrap();

    let mut bingocards: Vec<BingoCard> = Vec::new();

    while cloned.len() > 0 {
        let mut board_rows: Vec<String> = Vec::new();
        for i in 0..6 {
            let row = cloned.pop().unwrap();
            if i == 0 {
                continue;
            }

            board_rows.push(row)
        }

        bingocards.push(parse_board(board_rows));
    }

    // Parse input string;
    let numbers: Vec<i8> = numbers_from_line(&input, ',');
    let mut found_sum = -1;
    let mut found_number = -1;
    for number in numbers {
        for b in 0..bingocards.len() {
            bingocards[b] = update_board(&bingocards[b], number);
            let sum = check_board(&bingocards[b]);
            if sum != -1 {
                found_number = number;
                found_sum = sum;
                break;
            }
        }
        if found_sum != -1 {
            break;
        }
    }

    println!(
        "Part One: {} {} {}",
        found_sum,
        found_number,
        found_sum * i32::from(found_number)
    );
}

fn part2(lines: &Vec<String>) {
    let mut cloned = lines.clone();
    cloned.reverse();
    let input = cloned.pop().unwrap();

    let mut bingocards: Vec<BingoCard> = Vec::new();

    while cloned.len() > 0 {
        let mut board_rows: Vec<String> = Vec::new();
        for i in 0..6 {
            let row = cloned.pop().unwrap();
            if i == 0 {
                continue;
            }

            board_rows.push(row)
        }
        board_rows.reverse();
        bingocards.push(parse_board(board_rows));
    }

    // Parse input string;
    let numbers: Vec<i8> = numbers_from_line(&input, ',');
    let mut bingos: Vec<usize> = Vec::new();
    let mut last_bingo_number = 0;
    let mut last_bingo_sum = 0;

    for number in numbers {
        for b in 0..bingocards.len() {
            bingocards[b] = update_board(&bingocards[b], number);

            let sum = check_board(&bingocards[b]);

            if sum != -1 && !bingos.contains(&b) {
                last_bingo_number = number;
                last_bingo_sum = sum;
                bingos.push(b);
            }
        }
    }

    println!(
        "Part Two: {}",
        last_bingo_sum * i32::from(last_bingo_number)
    );
    std::process::exit(0);
}
