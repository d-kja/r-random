pub enum Move {
    UP,
    DOWN,
}

pub fn move_line(movement_type: Move, current_line: &mut u16, total_lines: u16) -> Option<String> {
    match movement_type {
        Move::UP => {
            if *current_line <= 1 {
                return None;
            }

            *current_line = *current_line - 1;

            Some(format!("{}", termion::cursor::Goto(1, *current_line)))
        }
        Move::DOWN => {
            if *current_line == total_lines - 1 {
                return None;
            }

            *current_line = *current_line + 1;

            Some(format!("{}", termion::cursor::Goto(1, *current_line)))
        }
    }
}

fn toggle_line(current_line: u16) {}
