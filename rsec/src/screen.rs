use std::fmt::format;

use owo_colors::OwoColorize;

use crate::config::CliConfig;

struct Section<T> {
    title: String,
    options: Vec<T>,
}

impl<T> Section<T> {
    fn new(title: &str, options: Vec<T>) -> Self {
        Self {
            title: title.to_owned(),
            options,
        }
    }
}

pub fn render(selected_line: u16) -> (String, u16) {
    let mut line_idx: u16 = 1;
    let mut lines = Vec::<String>::new();

    let sections: Vec<Section<&str>> = vec![
        Section::new("Language options", vec!["typescript", "javascript"]),
        Section::new("Style options", vec!["tailwind"]),
    ];

    for section in sections {
        if line_idx > 1 {
            let empty_line = format!("{}", termion::cursor::Goto(1, line_idx));

            add_line(&mut lines, &empty_line, &mut line_idx, selected_line);
        }

        add_line(
            &mut lines,
            &format!("# {}", section.title.cyan().bold()),
            &mut line_idx,
            selected_line,
        );

        for opt in section.options {
            let base_option = format!("[ ] {}", opt);
            let option_format = format!(" {}", base_option.dimmed());
            add_line(&mut lines, &option_format, &mut line_idx, selected_line);
        }
    }

    // Empty / Break line
    lines.push(format!("{}", termion::cursor::Goto(1, line_idx)));

    (lines.concat(), lines.len() as u16)
}

/// Creates a new line in the screen and moves the cursor
fn add_line(lines_arr: &mut Vec<String>, line: &str, idx: &mut u16, selected_line: u16) {
    let current_line = termion::cursor::Goto(1, *idx);
    let mut formatted_line = format!("{current_line}  {}", line);

    if selected_line == *idx {
        formatted_line = format!("{current_line}> {}", line.on_white().bold());
    }

    lines_arr.push(formatted_line);
    increment_idx(idx);
}

/// Simple incrementation, used to keep the code more semantic
fn increment_idx(idx: &mut u16) {
    *idx = *idx + 1_u16;
}
