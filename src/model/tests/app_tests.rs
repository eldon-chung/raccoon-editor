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
        assert_eq!(opened_text, "Testing Read!");

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

        assert!(Path::new(&file_path_string).exists());
        let opened_text = app.get_text_as_iter().join("");
        assert_eq!(opened_text, ""); // There should be nothing

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
        assert_eq!(saved_text, "Testing Write!");

        dir.close()?;
        Ok(())
    }
}
