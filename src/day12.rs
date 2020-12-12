pub fn part1(inp: String) {
    let moves = read_moves(&inp);

    let (x, y) = perform_moves(&moves);
    let distance = x.abs() + y.abs();

    println!("Manhattan distance from origin: {}", distance);
}

pub fn perform_moves(moves: &Vec<(char, i32)>) -> (i32, i32) {
    let mut x: i32 = 0; // x goes right
    let mut y: i32 = 0; // y goes up
    let mut azimuth: char = 'E';

    for movement in moves {
        let action = movement.0;
        let value = movement.1;
        match action {
            'R' | 'L' => {
                azimuth = turn(azimuth, action, value);
            }
            'F' => {
                let vector = get_forward_vector(azimuth, value);
                x += vector.0;
                y += vector.1;
            }
            'N' | 'E' | 'S' | 'W' => {
                let vector = get_forward_vector(action, value);
                x += vector.0;
                y += vector.1;
            }
            invalid => panic!("Unknown action: {}", invalid),
        };
    }

    (x, y)
}

pub fn get_forward_vector(azimuth: char, distance: i32) -> (i32, i32) {
    match azimuth {
        'E' => (distance, 0),
        'S' => (0, -distance),
        'W' => (-distance, 0),
        'N' => (0, distance),
        invalid => panic!("Unknown direction for forward vector: {}", invalid),
    }
}

pub fn turn(old_azimuth: char, direction: char, degrees: i32) -> char {
    let sign = match direction {
        'R' => 1,
        'L' => -1,
        _ => panic!("Unknown direction to turn into"),
    };

    let old_azimuth_degrees = match old_azimuth {
        'E' => 0,
        'S' => 90,
        'W' => 180,
        'N' => 270,
        invalid => panic!("Cannot convert this to degrees: {}", invalid),
    };

    let new_azimuth_degrees = (old_azimuth_degrees + (sign * degrees) + 360) % 360;

    let new_azimuth = match new_azimuth_degrees {
        0 => 'E',
        90 => 'S',
        180 => 'W',
        270 => 'N',
        invalid => panic!("Cannot convert this to E/S/W/N: {}", invalid),
    };

    new_azimuth
}

pub fn part2(_inp: String) {}

fn parse_move(line: &str) -> (char, i32) {
    let action = line.chars().nth(0).unwrap();
    let value = line[1..].parse::<i32>().unwrap();
    (action, value)
}

fn read_moves(inp: &str) -> Vec<(char, i32)> {
    inp.split("\n")
        .filter(|line| line.len() > 0)
        .map(|x| parse_move(x))
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn parse_move_sample1() {
        let line = "R90";
        let expected_result = ('R', 90);

        let result = parse_move(&line);

        assert_eq!(result, expected_result);
    }

    #[test]
    pub fn read_moves_sample1() {
        let input = "R90\nF23\nN42";
        let expected_result = vec![('R', 90), ('F', 23), ('N', 42)];

        let result = read_moves(&input);

        assert_eq!(result, expected_result);
    }

    #[test]
    pub fn turn_wraparound_right() {
        let old = 'N';
        let direction = 'R';
        let degrees = 90;
        let expected_result = 'E';

        let result = turn(old, direction, degrees);

        assert_eq!(result, expected_result);
    }

    #[test]
    pub fn turn_wraparound_left() {
        let old = 'E';
        let direction = 'L';
        let degrees = 180;
        let expected_result = 'W';

        let result = turn(old, direction, degrees);

        assert_eq!(result, expected_result);
    }

    #[test]
    pub fn perform_moves_sample() {
        let moves = "F10\nN3\nF7\nR90\nF11\n";

        let expected_result = (17, -8);

        let result = perform_moves(&read_moves(moves));
        assert_eq!(result, expected_result);
    }
}
