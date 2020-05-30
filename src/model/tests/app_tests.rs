#[cfg(test)]
mod app_tests {
    use super::super::*;
    use std::fs::File;
    use std::io::{self, Write};
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn init_new_file_ok() -> io::Result<()> {
        // Prepare the files
        let dir = tempdir()?;
        let file_path = dir.path().join("temp.txt");
        let file_path_string = file_path.to_string_lossy().into_owned();

        // Prepare the application
        App::init_new_file(file_path_string.clone());

        assert!(Path::new(&file_path_string).exists());

        dir.close()?;
        Ok(())
    }

    #[test]
    fn open_file_sucess() -> io::Result<()> {
        // Prepare the files
        let dir = tempdir()?;
        let file_path = dir.path().join("temp.txt");
        let file_path_string = file_path.to_string_lossy().into_owned();

        let mut file = File::create(file_path.clone())?;
        write!(file, "Testing Read!")?;

        drop(file);

        // Prepare the application
        let mut app = App::new();
        app.set_app_mode(AppMode::Command(CommandMode::Read));
        app.command_buffer = Buffer::with_contents(file_path_string);

        app.open_file();

        let opened_text = app.get_text_as_iter().join("");
        assert_eq!(opened_text, "Testing Read!", "Mismatch between opened text and what is in the buffer");

        dir.close()?;
        Ok(())
    }

    #[test]
    fn open_file_not_created_yet() -> io::Result<()> {
        // Prepare the files
        let dir = tempdir()?;
        let file_path = dir.path().join("temp.txt");
        let file_path_string = file_path.to_string_lossy().into_owned();

        // Prepare the application
        let mut app = App::new();
        app.set_app_mode(AppMode::Command(CommandMode::Read));
        app.command_buffer = Buffer::with_contents(file_path_string.clone());

        app.open_file();

        assert!(Path::new(&file_path_string).exists(), "A new file is not created");
        let opened_text = app.get_text_as_iter().join("");
        assert_eq!(opened_text, ""); // There should be nothing

        let app_cursor = app.cursor_main();
        assert_eq!(app_cursor.node_idx, 0, "app_cursor.node_idx mismatch");
        assert_eq!(app_cursor.node_offset, 0, "app_cursor.node_offset mismatch");
        assert_eq!(app_cursor.line_idx, 0, "app_cursor.line_idx mismatch");
        assert_eq!(app_cursor.line_offset, 0, "app_cursor.line_offset mismatch");

        dir.close()?;
        Ok(())
    }

    #[test]
    fn save_file_success() -> io::Result<()> {
        // Prepare the files
        let dir = tempdir()?;
        let file_path = dir.path().join("temp.txt");
        let file_path_string = file_path.to_string_lossy().into_owned();

        // Prepare the application
        let mut app = App::new();
        app.set_app_mode(AppMode::Command(CommandMode::Write));
        app.command_buffer = Buffer::with_contents(file_path_string);
        app.buffer = Buffer::with_contents(String::from("Testing Write!"));

        app.save_file();

        let saved_text = fs::read_to_string(file_path)?;
        assert_eq!(saved_text, "Testing Write!", "Mismatch between the text that has been saved and what was in the buffer");

        dir.close()?;
        Ok(())
    }
}
