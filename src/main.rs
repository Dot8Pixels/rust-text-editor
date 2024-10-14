use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::fs::File;
use std::io::{self, Read, Write};

fn save_to_file(filename: &str, buffer: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(buffer.as_bytes())?;
    Ok(())
}

fn load_from_file(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn main() -> io::Result<()> {
    let _raw_mode_guard = RawModeGuard::new()?; // Ensures raw mode is disabled on exit

    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;

    let filename = "test_file.txt"; // You can modify this or ask the user to input a file name
    let mut buffer = load_from_file(filename).unwrap_or_default();
    execute!(stdout, Print(&buffer))?;

    let mut cursor_x = buffer.lines().last().unwrap_or("").len(); // Track current cursor position (column)
    let mut cursor_y = buffer.lines().count(); // Track current cursor line (row)

    loop {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Char(c) => {
                            // Handle character input
                            buffer.push(c);
                            cursor_x += 1;
                            execute!(stdout, Print(c))?;
                        }
                        KeyCode::Enter => {
                            // Handle Enter key to create a new line
                            buffer.push('\n'); // Add newline character to buffer
                            cursor_x = 0;
                            cursor_y += 1;
                            execute!(stdout, cursor::MoveToNextLine(1))?; // Move to the next line
                        }
                        KeyCode::Backspace => {
                            // Handle backspace
                            if cursor_x > 0 {
                                // Normal backspace within the line
                                buffer.pop();
                                cursor_x -= 1;
                                execute!(
                                    stdout,
                                    cursor::MoveLeft(1),
                                    Print(" "),
                                    cursor::MoveLeft(1)
                                )?;
                            } else if cursor_y > 1 {
                                // Move to the previous line
                                buffer.pop(); // Remove the newline character
                                cursor_y -= 1;

                                // Find the length of the previous line
                                let previous_line_length =
                                    buffer.lines().nth(cursor_y - 1).unwrap_or("").len();
                                cursor_x = previous_line_length;

                                // Move the cursor to the end of the previous line
                                execute!(
                                    stdout,
                                    cursor::MoveUp(1),
                                    cursor::MoveRight(previous_line_length as u16),
                                    Print(" "), // Clear the space from the current position
                                    cursor::MoveLeft(1)
                                )?;
                            }
                        }
                        KeyCode::Esc => {
                            // Save and exit
                            save_to_file(filename, &buffer)?;
                            break;
                        }
                        KeyCode::Char('s') if key_event.modifiers == KeyModifiers::CONTROL => {
                            // Ctrl+S to save without exiting
                            save_to_file(filename, &buffer)?;
                            execute!(stdout, cursor::MoveTo(0, 0), Print("File Saved!"))?;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(())
}

struct RawModeGuard;

impl RawModeGuard {
    fn new() -> io::Result<Self> {
        // Changed to io::Result
        enable_raw_mode()?; // Enable raw mode
        Ok(RawModeGuard)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode(); // Disable raw mode on drop
    }
}
