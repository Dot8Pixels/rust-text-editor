use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
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
    enable_raw_mode()?; // Enables raw mode for the terminal
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;

    // Specify the file to load and save
    let filename = "test_file.txt"; // You can modify this or ask the user to input a file name

    // Try loading the file, if it exists
    let mut buffer = load_from_file(filename).unwrap_or_default();

    // Display the loaded content in the terminal
    execute!(stdout, Print(&buffer))?;

    loop {
        // Check if a key event is available
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                // Filter out key release and repeat events, handle only key press events
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Char(c) => {
                            // Handle character input
                            buffer.push(c);
                            execute!(stdout, Print(c))?;
                        }
                        KeyCode::Backspace => {
                            // Handle backspace
                            if !buffer.is_empty() {
                                buffer.pop();
                                execute!(
                                    stdout,
                                    cursor::MoveLeft(1),
                                    Print(" "),
                                    cursor::MoveLeft(1)
                                )?;
                            }
                        }
                        KeyCode::Esc => {
                            // Save the buffer to the file before exiting
                            save_to_file(filename, &buffer)?;
                            disable_raw_mode()?;
                            break;
                        }
                        KeyCode::Enter => {
                            // Handle Enter key to create a new line
                            buffer.push('\n'); // Add newline character to buffer
                            execute!(stdout, cursor::MoveToNextLine(1))?; // Move to the next line
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Result;
    use tempfile::NamedTempFile;

    #[test]
    fn test_save_to_file() -> Result<()> {
        // Create a temporary file
        let temp_file = NamedTempFile::new()?;
        let filename = temp_file.path().to_str().unwrap(); // Get the path

        // Data to save
        let data = "Hello, Rust!";

        // Save data to the file
        save_to_file(filename, data)?;

        // Check that the file contains the correct data
        let saved_content = fs::read_to_string(filename)?;
        assert_eq!(saved_content, data);

        Ok(())
    }

    #[test]
    fn test_load_from_file() -> Result<()> {
        // Create a temporary file with predefined content
        let mut temp_file = NamedTempFile::new()?;

        // Extract the file path before mutably borrowing temp_file to write data
        let filename = temp_file.path().to_str().unwrap().to_string();

        let data = "Rust programming!";
        temp_file.write_all(data.as_bytes())?;

        // Load the content from the file
        let loaded_content = load_from_file(&filename)?;

        // Check if the loaded content matches the predefined content
        assert_eq!(loaded_content, data);

        Ok(())
    }

    #[test]
    fn test_load_from_non_existent_file() {
        // Try loading from a non-existent file
        let result = load_from_file("non_existent_file.txt");

        // It should return an error
        assert!(result.is_err());
    }
}
