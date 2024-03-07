use crossterm::cursor::SetCursorStyle;
use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::style::{Color, ResetColor, SetForegroundColor};
use crossterm::terminal::{self, Clear, ClearType};
use crossterm::{cursor, QueueableCommand};
use std::cmp::min;
use std::io::{stdout, Write};
use std::ops::Div;
use std::thread::sleep;
use std::time::Duration;
use ueberzug::{Scalers, UeConf};

const MAX_OPTIONS: usize = 25;
const COLOR: Color = Color::Yellow;

pub fn choose_with_images(options: &Vec<String>, imgs_path: Vec<String>) -> Result<usize, ()> {
    let mut stdout = stdout();
    let mut last_option = min(MAX_OPTIONS, options.len());
    let mut first_option = 0;
    let mut previous_option = 0;
    let mut current_option = 0;

    let (mut width, mut height) = terminal::size().unwrap();

    terminal::enable_raw_mode().unwrap();
    stdout.queue(SetCursorStyle::SteadyBar).unwrap();

    write_options(&mut stdout, options, first_option, last_option, 0);
    stdout.queue(cursor::MoveTo(0, 0)).unwrap();

    let mut img_width = width / 2;
    let padding_right = 2;

    let a = ueberzug::Ueberzug::new();
    a.draw(&UeConf {
        identifier: &options[current_option],
        path: &imgs_path[current_option],
        x: img_width - padding_right,
        y: height / 4,
        width: Some(width / 2),
        height: Some(height / 2),
        scaler: Some(Scalers::FitContain),
        ..Default::default()
    });

    loop {
        while poll(Duration::ZERO).unwrap() {
            match read().unwrap() {
                Event::Resize(nw, nh) => {
                    width = nw;
                    height = nh;
                    img_width = nw / 2;

                    let identifier = &options[current_option];
                    a.clear(identifier);

                    a.draw(&UeConf {
                        identifier: &options[current_option],
                        path: &imgs_path[current_option],
                        x: img_width - padding_right,
                        y: height / 4,
                        width: Some(width / 2),
                        height: Some(height / 2),
                        scaler: Some(Scalers::FitContain),
                        ..Default::default()
                    });
                }

                Event::Key(event) => match event.code {
                    KeyCode::Up => {
                        let (_, row) = crossterm::cursor::position().unwrap();
                        let cursor_pos = row;

                        if row == 0 && first_option == 0 {
                            break;
                        }

                        current_option -= 1;

                        if options.len() > MAX_OPTIONS {
                            if row == 0 && first_option != 0 {
                                first_option -= 1;
                                last_option -= 1;
                                write_options(
                                    &mut stdout,
                                    options,
                                    first_option,
                                    last_option,
                                    cursor_pos,
                                );
                                stdout.queue(cursor::MoveTo(0, row)).unwrap();
                            } else {
                                write_options(
                                    &mut stdout,
                                    options,
                                    first_option,
                                    last_option,
                                    cursor_pos - 1,
                                );
                                stdout.queue(cursor::MoveTo(0, row)).unwrap();
                                stdout.queue(cursor::MoveToPreviousLine(1)).unwrap();
                            }
                        }
                        if first_option == 0 && options.len() <= MAX_OPTIONS {
                            write_options(
                                &mut stdout,
                                options,
                                first_option,
                                last_option,
                                cursor_pos - 1,
                            );
                            stdout.queue(cursor::MoveTo(0, row)).unwrap();
                            stdout.queue(cursor::MoveToPreviousLine(1)).unwrap();
                        }
                    }
                    KeyCode::Down => {
                        let (_, row) = crossterm::cursor::position().unwrap();
                        let cursor_pos = row;

                        let middle_row_without_all_options: bool =
                            row != (MAX_OPTIONS.div(2)) as u16 && last_option < options.len();
                        let second_to_last_row_with_all_options: bool =
                            row != (MAX_OPTIONS - 1) as u16 && last_option == options.len();
                        let last_row_with_all_options: bool =
                            row == (MAX_OPTIONS - 1) as u16 && last_option == options.len();

                        if options.len() > MAX_OPTIONS {
                            if last_row_with_all_options {
                                break;
                            }

                            current_option += 1;

                            if middle_row_without_all_options || second_to_last_row_with_all_options
                            {
                                write_options(
                                    &mut stdout,
                                    options,
                                    first_option,
                                    last_option,
                                    cursor_pos + 1,
                                );
                                stdout.queue(cursor::MoveTo(0, row)).unwrap();
                                stdout.queue(cursor::MoveToNextLine(1)).unwrap();
                            } else {
                                first_option += 1;
                                last_option += 1;
                                write_options(
                                    &mut stdout,
                                    options,
                                    first_option,
                                    last_option,
                                    cursor_pos,
                                );
                                stdout.queue(cursor::MoveTo(0, cursor_pos)).unwrap();
                            }
                        }
                        if options.len() <= MAX_OPTIONS && row != (last_option - 1) as u16 {
                            current_option += 1;

                            write_options(
                                &mut stdout,
                                options,
                                first_option,
                                last_option,
                                cursor_pos + 1,
                            );
                            stdout.queue(cursor::MoveTo(0, row)).unwrap();
                            stdout.queue(cursor::MoveToNextLine(1)).unwrap();
                        }
                    }
                    KeyCode::Enter => {
                        stdout.queue(ResetColor).unwrap();
                        stdout.queue(Clear(ClearType::All)).unwrap();
                        stdout.queue(cursor::MoveTo(0, 0)).unwrap();
                        terminal::disable_raw_mode().unwrap();
                        return Ok(current_option);
                    }
                    _ => {}
                },

                _ => {}
            }

            if current_option != previous_option {
                let identifier = &options[previous_option];
                previous_option = current_option;
                a.clear(identifier);

                a.draw(&UeConf {
                    identifier: &options[current_option],
                    path: &imgs_path[current_option],
                    x: img_width - padding_right,
                    y: height / 4,
                    width: Some(width / 2),
                    height: Some(height / 2),
                    scaler: Some(Scalers::FitContain),
                    ..Default::default()
                });
            }
        }
        stdout.flush().unwrap();
        sleep(Duration::from_millis(33))
    }
}

fn write_options(
    stdout: &mut impl Write,
    options: &[String],
    first_option: usize,
    last_option: usize,
    cursor_pos: u16,
) {
    let mut row: u16 = 0;

    stdout.queue(cursor::MoveTo(0, row)).unwrap();
    stdout.queue(Clear(ClearType::All)).unwrap();

    for option in &options[first_option..last_option] {
        let selected_option = format!("> {}", option);
        let unselected_option = format!("  {}", option);
        if row == cursor_pos {
            stdout.queue(SetForegroundColor(COLOR)).unwrap();
            stdout.write_all(selected_option.as_bytes()).unwrap();
        } else {
            stdout.queue(ResetColor).unwrap();
            stdout.write_all(unselected_option.as_bytes()).unwrap();
        }
        row += 1;
        stdout.queue(cursor::MoveTo(0, row)).unwrap();
    }
}
