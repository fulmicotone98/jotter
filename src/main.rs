use std::process::ExitCode;

use crate::editor::Editor;

mod document;
mod editor;
mod error;
mod event;
mod filetype;
mod row;
mod sys;

pub const JOTTER_VERSION: &str = "0.1.0";

fn main() -> ExitCode {
    let mut editor = match Editor::default() {
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
