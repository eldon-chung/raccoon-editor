#[cfg(test)]
mod app_tests {
    use super::super::*;
    use std::fs::{self, File};
    use std::io::{self, Write};
    use std::os::unix::fs::PermissionsExt;
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
        let args: Vec<String> = Vec::new();
        let mut app = App::new(&args);
        app.set_mode(AppMode::Command(CommandMode::Read));
        app.command_buffer = Buffer::with_contents(file_path_string);

        app.open_file();

        let opened_text = app.get_buffer_text();
        assert_eq!(
            opened_text, "Testing Read!",
            "Mismatch between opened text and what is in the buffer"
        );

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
        let args: Vec<String> = Vec::new();
        let mut app = App::new(&args);
        app.set_mode(AppMode::Command(CommandMode::Read));
        app.command_buffer = Buffer::with_contents(file_path_string.clone());

        app.open_file();

        assert!(
            Path::new(&file_path_string).exists(),
            "A new file is not created"
        );
        let opened_text = app.get_buffer_text();
        assert_eq!(opened_text, ""); // There should be nothing

        dir.close()?;
        Ok(())
    }

    #[test]
    #[should_panic(expected = "Permission denied")]
    fn open_writeonly_file_fail() {
        // Doesn't return io::Result<()> like other tests because should_panic must return unit
        // Prepare the files
        let dir = tempdir().expect("Failed to create tempdir");
        let file_path = dir.path().join("temp.txt");
        let file_path_string = file_path.to_string_lossy().into_owned();
        let file = File::create(file_path.clone()).expect("Failed to create file");

        // Make the file become writeonly
        let metadata = file.metadata().expect("Failed to get metadata");
        let mut permissions = metadata.permissions();

        // NOTE: set_mode is supported on Unix only
        // See https://doc.rust-lang.org/std/os/unix/fs/trait.PermissionsExt.html
        permissions.set_mode(0o244); // -w-r--r-- permission, with writeonly for the user
        fs::set_permissions(file_path, permissions).expect("Failed to set permissions");

        // Prepare the application
        let args: Vec<String> = Vec::new();
        let mut app = App::new(&args);
        app.set_mode(AppMode::Command(CommandMode::Read));
        app.command_buffer = Buffer::with_contents(file_path_string.clone());

        app.open_file();
    }

    #[test]
    fn save_file_success() -> io::Result<()> {
        // Prepare the files
        let dir = tempdir()?;
        let file_path = dir.path().join("temp.txt");
        let file_path_string = file_path.to_string_lossy().into_owned();

        // Prepare the application
        let args: Vec<String> = Vec::new();
        let mut app = App::new(&args);
        app.set_mode(AppMode::Command(CommandMode::Write));
        app.command_buffer = Buffer::with_contents(file_path_string);
        app.buffer = Buffer::with_contents(String::from("Testing Write!"));

        app.save_file();

        let saved_text = fs::read_to_string(file_path)?;
        assert_eq!(
            saved_text, "Testing Write!",
            "Mismatch between the text that has been saved and what was in the buffer"
        );

        dir.close()?;
        Ok(())
    }

    #[test]
    #[should_panic(expected = "Unable to write file")]
    fn save_readonly_file_fail() {
        // Doesn't return io::Result<()> like other tests because should_panic must return unit
        // Prepare the files
        let dir = tempdir().expect("Failed to create tempdir");
        let file_path = dir.path().join("temp.txt");
        let file_path_string = file_path.to_string_lossy().into_owned();
        let file = File::create(file_path.clone()).expect("Failed to create file");

        // Make the file become readonly
        let metadata = file.metadata().expect("Failed to get metadata");
        let mut permissions = metadata.permissions();
        permissions.set_readonly(true);
        fs::set_permissions(file_path, permissions).expect("Failed to set permissions");

        // Prepare the application
        let args: Vec<String> = Vec::new();
        let mut app = App::new(&args);
        app.set_mode(AppMode::Command(CommandMode::Write));
        app.command_buffer = Buffer::with_contents(file_path_string.clone());
        app.buffer = Buffer::with_contents(String::from("Testing Write, this should fail!"));

        app.save_file();
    }

    #[test]
    fn get_text_in_edit_mode() {
        // Prepare the application
        let args: Vec<String> = Vec::new();
        let mut app = App::new(&args);
        app.buffer = Buffer::with_contents(String::from("Piece Tables!"));

        let text_as_iter = app.get_text_based_on_mode();
        assert_eq!(text_as_iter, vec!["Piece Tables!"]);
    }

    #[test]
    fn get_text_in_command_write_mode() {
        // Prepare the application
        let args: Vec<String> = Vec::new();
        let mut app = App::new(&args);
        app.set_mode(AppMode::Command(CommandMode::Write));
        app.command_buffer = Buffer::with_contents(String::from("Gap Buffers!"));

        let text_as_iter = app.get_text_based_on_mode();
        assert_eq!(text_as_iter, vec!["Gap Buffers!"]);
    }

    #[test]
    fn get_text_in_command_read_mode() {
        // Prepare the application
        let args: Vec<String> = Vec::new();
        let mut app = App::new(&args);
        app.set_mode(AppMode::Command(CommandMode::Read));
        app.command_buffer = Buffer::with_contents(String::from("Rope!"));

        let text_as_iter = app.get_text_based_on_mode();
        assert_eq!(text_as_iter, vec!["Rope!"]);
    }

    #[test]
    fn get_buffer_text_test() {
        // Prepare the application
        let args: Vec<String> = Vec::new();
        let mut app = App::new(&args);
        app.buffer = Buffer::with_contents(String::from("Piece Tables!"));

        let text = app.get_buffer_text();
        assert_eq!(text, "Piece Tables!");
    }

    #[test]
    fn get_command_buffer_text_test() {
        // Prepare the application
        let args: Vec<String> = Vec::new();
        let mut app = App::new(&args);
        app.command_buffer = Buffer::with_contents(String::from("Gap Buffers!"));

        let text = app.get_command_buffer_text();
        assert_eq!(text, "Gap Buffers!");
    }

    #[test]
    fn handle_regular_save_with_no_filepath() {
        // Prepare the application
        let args: Vec<String> = Vec::new();
        let mut app = App::new(&args);

        app.handle_regular_save();
        assert_eq!(app.mode(), AppMode::Command(CommandMode::Write));
    }

    #[test]
    fn handle_regular_save_with_a_filepath() -> io::Result<()> {
        // Prepare the files
        let dir = tempdir()?;
        let file_path = dir.path().join("temp.txt");
        let file_path_string = file_path.to_string_lossy().into_owned();

        // Prepare the application
        let args: Vec<String> = Vec::new();
        let mut app = App::new(&args);
        app.set_mode(AppMode::Command(CommandMode::Write));
        app.command_buffer = Buffer::with_contents(file_path_string);
        app.buffer = Buffer::with_contents(String::from("Testing Write!"));

        app.handle_regular_save();
        assert_eq!(app.mode(), AppMode::Edit);

        let saved_text = fs::read_to_string(file_path)?;
        assert_eq!(
            saved_text, "Testing Write!",
            "Mismatch between the text that has been saved and what was in the buffer"
        );

        Ok(())
    }

    #[test]
    fn handle_save_as_new_file_test() {
        // Prepare the application
        let args: Vec<String> = Vec::new();
        let mut app = App::new(&args);

        app.handle_save_as_new_file();
        assert_eq!(app.mode(), AppMode::Command(CommandMode::Write));
    }

    #[test]
    fn add_char_in_edit_mode() {
        // Prepare the application
        // Note that both buffers are empty and we start the app in edit mode
        let args: Vec<String> = Vec::new();
        let mut app = App::new(&args);

        app.add_char('a');
        app.add_char('b');

        assert_eq!("ab", app.buffer.as_str());
        assert_eq!("", app.command_buffer.as_str()); // Command buffer should still be the same
    }

    #[test]
    fn add_char_in_command_write_mode() {
        // Prepare the application
        // Note that both buffers are empty and we start the app in edit mode
        let args: Vec<String> = Vec::new();
        let mut app = App::new(&args);
        app.set_mode(AppMode::Command(CommandMode::Write));

        app.add_char('a');
        app.add_char('b');

        assert_eq!("", app.buffer.as_str()); // Buffer should still be the same
        assert_eq!("ab", app.command_buffer.as_str());
    }

    #[test]
    fn add_char_in_command_read_mode() {
        // Prepare the application
        // Note that both buffers are empty and we start the app in edit mode
        let args: Vec<String> = Vec::new();
        let mut app = App::new(&args);
        app.set_mode(AppMode::Command(CommandMode::Read));

        app.add_char('a');
        app.add_char('b');

        assert_eq!("", app.buffer.as_str()); // Buffer should still be the same
        assert_eq!("ab", app.command_buffer.as_str());
    }
}
