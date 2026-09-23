use std::process::ExitCode;

use crate::editor::Editor;

mod editor;
mod error;
mod events;
mod sys;

fn main() -> ExitCode {
    let editor = match Editor::new() {
        Ok(editor) => editor,
        Err(e) => {
            eprintln!("ERROR: {e}\r");
            return ExitCode::FAILURE;
        }
    };

    if let Err(err_run) = editor.run() {
        eprintln!("ERROR: {err_run}\r");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
