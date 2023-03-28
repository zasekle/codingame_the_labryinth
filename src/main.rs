
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct PointAndPath {
    point: Point,
    moves: Vec<String>,
}

#[derive(PartialEq, Debug)]
enum IncrementReturnVal {
    Incremented,
    PathCompleted,
    Failure,
}

fn main() {
    let mut board: Vec<Vec<u8>> = Vec::new();

    board.push("??????????????????????????????".as_bytes().to_vec());
    board.push("#..............???????????????".as_bytes().to_vec());
    board.push("#.#############???????????????".as_bytes().to_vec());
    board.push("#.....T........???????????????".as_bytes().to_vec());
    board.push("#.......................#.#..#".as_bytes().to_vec());
    board.push("#.#######################.#..#".as_bytes().to_vec());
    board.push("#.....##......##......#....###".as_bytes().to_vec());
    board.push("#...####..##..##..##..#..#...#".as_bytes().to_vec());
    board.push("#.........##......##.....#.C.#".as_bytes().to_vec());
    board.push("##############################".as_bytes().to_vec());

    let mut point = Point{x: 0, y: 0};
    for (y, vec) in board.iter().enumerate() {
        for (x, &c) in vec.iter().enumerate() {
            if c == b'C' {
                point.x = x as i32;
                point.y = y as i32;
            }
        }
    }

    let moves = find_shortest_route(
        board.clone(),
        point.x,
        point.y,
        b'T'
    );

    if moves.is_empty() {
        println!("NO PATH FOUND!");
    } else {
        for mov in moves {
            println!("{mov}");
        }
        println!("FINISHED!");
    }

}

fn find_shortest_route(
    mut board_copy: Vec<Vec<u8>>,
    start_x: i32,
    start_y: i32,
    end_char: u8
) -> Vec<String> {

    let mut points: Vec<PointAndPath> = Vec::from([PointAndPath { point: Point { x: start_x, y: start_y }, moves: Vec::new() }]);
    let mut moves: Vec<String> = Vec::new();

    'outer: while !points.is_empty() {
        let mut points_copy = points;
        points = Vec::new();

        while !points_copy.is_empty() {
            let point_and_path = points_copy.pop().expect("pop failed");

            for i in -1..=1 {
                for j in -1..=1 {
                    //only want directions {UP DOWN LEFT RIGHT} skip all other combinations
                    if (i != 0 && j != 0) || (i == 0 && j == 0) {
                        continue;
                    }

                    let incremented = increment_if_valid_point(
                        &mut board_copy,
                        point_and_path.point.x + i,
                        point_and_path.point.y + j,
                        &end_char
                    );

                    if incremented != IncrementReturnVal::Failure {
                        let mut new_moves = Vec::from(point_and_path.moves.clone());

                        let move_name =
                            if i == 1 {
                                "RIGHT"
                            } else if i == -1 {
                                "LEFT"
                            } else if j == 1 {
                                "DOWN"
                            } else { //j == -1
                                "UP"
                            };

                        new_moves.push(move_name.to_string());

                        if incremented == IncrementReturnVal::PathCompleted {
                            moves = new_moves;
                            break 'outer;
                        } else { //IncrementReturnVal::Incremented
                            points.push(
                                PointAndPath {
                                    point: Point { x: point_and_path.point.x + i, y: point_and_path.point.y + j },
                                    moves: new_moves,
                                }
                            );
                        }
                    }
                }
            }
        }
    }

    moves
}

fn increment_if_valid_point(
    board_copy: &mut Vec<Vec<u8>>,
    x: i32,
    y: i32,
    end_char: &u8
) -> IncrementReturnVal {
    if y < 0
        || board_copy.len() <= y as usize
        || board_copy.is_empty()
        || x < 0
        || board_copy[0].len() <= x as usize
        || (board_copy[y as usize][x as usize] != b'.' && board_copy[y as usize][x as usize] != *end_char)
    {
        IncrementReturnVal::Failure
    } else if board_copy[y as usize][x as usize] == *end_char {
        IncrementReturnVal::PathCompleted
    } else {
        board_copy[y as usize][x as usize] = b'U';
        IncrementReturnVal::Incremented
    }
}