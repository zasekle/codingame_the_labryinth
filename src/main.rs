use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let r = parse_input!(inputs[0], i8); // number of rows.
    let c = parse_input!(inputs[1], i8); // number of columns.
    let a = parse_input!(inputs[2], i8); // number of rounds between the time the alarm countdown is activated and the time the alarm goes off.

    let mut unexplored_moves = Vec::<String>::new();
    let mut control_room_moves = Vec::<String>::new();

    let mut start_x = -1;
    let mut start_y = -1;

    let mut c_x = -1;
    let mut c_y = -1;

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let kr = parse_input!(inputs[0], i32); // row where Rick is located.
        let kc = parse_input!(inputs[1], i32); // column where Rick is located.
        let mut board: Vec<Vec<u8>> = Vec::new();
        for _ in 0..r as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            // C of the characters in '#.TC?' (i.e. one line of the ASCII maze).
            board.push(input_line.trim().as_bytes().to_vec());
        }

        board[kr as usize][kc as usize] = b'K';

        if start_x == -1 && start_y == -1 {
            start_x = kc;
            start_y = kr;
        }

        if control_room_moves.is_empty() {

            let mut next_unexplored_moves = Vec::<String>::new();
            let mut closest_unexplored = -1;
            'outer: for (y, vec) in board.iter().enumerate() {
                for (x, &c) in vec.iter().enumerate() {
                    if c == b'?' && unexplored_moves.is_empty() {
                        let mut new_board = board.clone();

                        new_board[y][x] = b'Q';

                        let temp_unexplored_moves = find_shortest_route(
                            new_board,
                            kc as i32,
                            kr as i32,
                            b'Q'
                        );

                        if !temp_unexplored_moves.is_empty()
                            &&
                            (closest_unexplored < 0
                                || temp_unexplored_moves.len() < closest_unexplored as usize) {

                            closest_unexplored = temp_unexplored_moves.len() as i32;

                            next_unexplored_moves = temp_unexplored_moves;
                            next_unexplored_moves.pop();
                            next_unexplored_moves.reverse();
                        }
                    } else if c == b'C' {

                        c_x = x as i32;
                        c_y = y as i32;

                        let temp_control_room_moves = find_shortest_route(
                            board.clone(),
                            start_x,
                            start_y,
                            b'C'
                        );

                        //make sure can get to the start from the control room in the correct num moves
                        if !temp_control_room_moves.is_empty() && temp_control_room_moves.len() <= a as usize {

                            control_room_moves = find_shortest_route(
                                board.clone(),
                                kc as i32,
                                kr as i32,
                                b'C'
                            );

                            control_room_moves.reverse();

                            break 'outer;
                        }
                    }
                }
            }
            if unexplored_moves.is_empty() {
                unexplored_moves = next_unexplored_moves;
            }
        }

        let mut ran_control_room_move = false;
        let next_move = if !control_room_moves.is_empty() {
            ran_control_room_move = true;
            control_room_moves.pop().expect("pop up top fail")
        } else {
            unexplored_moves.pop().expect("pop up top fail")
        };

        println!("{next_move}");

        if ran_control_room_move && control_room_moves.is_empty() {

            control_room_moves = find_shortest_route(
                board.clone(),
                c_x,
                c_y,
                b'T'
            );

            control_room_moves.reverse();
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct PointAndPath {
    point: Point,
    moves: Vec<String>,
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum IncrementReturnVal {
    Incremented,
    PathCompleted,
    Failure,
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
        || (board_copy[y as usize][x as usize] != b'.'
        && board_copy[y as usize][x as usize] != b'T'
        && board_copy[y as usize][x as usize] != b'K'
        && board_copy[y as usize][x as usize] != *end_char)
    {
        IncrementReturnVal::Failure
    } else if board_copy[y as usize][x as usize] == *end_char {
        IncrementReturnVal::PathCompleted
    } else {
        board_copy[y as usize][x as usize] = b'U';
        IncrementReturnVal::Incremented
    }
}