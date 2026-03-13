pub mod app;
pub mod ui;

use std::io::{self, stdout};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

use crate::commands;
use app::{App, DumpField, Screen};
use ui::draw_ui;

/// RAII guard that ensures the terminal is restored on drop (including panics).
struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen);
    }
}

/// Run the interactive TUI.
pub fn run_tui() -> Result<()> {
    enable_raw_mode()?;
    execute!(
        stdout(),
        EnterAlternateScreen,
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
    )?;
    let _guard = TerminalGuard;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    main_loop(&mut terminal, &mut app)
}

fn main_loop(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    loop {
        terminal.clear()?;
        terminal.draw(|frame| draw_ui(frame, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match &app.screen {
                Screen::Menu => handle_menu(key.code, app),
                Screen::StartInput => handle_start_input(key.code, app),
                Screen::FinishInput => handle_finish_input(key.code, app),
                Screen::DumpConfig => handle_dump_config(key.code, app),
                Screen::ResultView => handle_result(key.code, app),
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn handle_menu(key: KeyCode, app: &mut App) {
    match key {
        KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Up | KeyCode::Char('k') => {
            if app.menu_index > 0 {
                app.menu_index -= 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.menu_index < App::menu_items().len() - 1 {
                app.menu_index += 1;
            }
        }
        KeyCode::Enter => match app.menu_index {
            0 => app.screen = Screen::StartInput,
            1 => app.screen = Screen::FinishInput,
            2 => {
                app.screen = Screen::DumpConfig;
                app.dump_field = DumpField::Branch;
            }
            3 => app.should_quit = true,
            _ => {}
        },
        _ => {}
    }
}

fn handle_start_input(key: KeyCode, app: &mut App) {
    match key {
        KeyCode::Esc => {
            app.screen = Screen::Menu;
            app.input.clear();
        }
        KeyCode::Enter => {
            if !app.input.is_empty() {
                let branch = app.input.clone();
                app.input.clear();
                app.output_log.clear();
                app.output_log
                    .push(format!("Running: git-workflow start {}", branch));
                execute_start(app, &branch);
                app.screen = Screen::ResultView;
            }
        }
        KeyCode::Backspace => {
            app.input.pop();
        }
        KeyCode::Char(c) => {
            app.input.push(c);
        }
        _ => {}
    }
}

fn handle_finish_input(key: KeyCode, app: &mut App) {
    match key {
        KeyCode::Esc => {
            app.screen = Screen::Menu;
            app.input.clear();
        }
        KeyCode::Enter => {
            if !app.input.is_empty() {
                let title = app.input.clone();
                app.input.clear();
                app.output_log.clear();
                app.output_log
                    .push(format!("Running: git-workflow finish \"{}\"", title));
                execute_finish(app, &title);
                app.screen = Screen::ResultView;
            }
        }
        KeyCode::Backspace => {
            app.input.pop();
        }
        KeyCode::Char(c) => {
            app.input.push(c);
        }
        _ => {}
    }
}

fn handle_dump_config(key: KeyCode, app: &mut App) {
    match key {
        KeyCode::Esc => {
            app.screen = Screen::Menu;
            app.reset_dump();
        }
        KeyCode::Tab | KeyCode::Down => {
            app.dump_field = app.dump_field.next();
        }
        KeyCode::BackTab | KeyCode::Up => {
            app.dump_field = app.dump_field.prev();
        }
        KeyCode::Enter => {
            if matches!(app.dump_field, DumpField::Execute) {
                app.output_log.clear();
                app.output_log
                    .push("Running: git-workflow dump ...".to_string());
                execute_dump(app);
                app.screen = Screen::ResultView;
                return;
            }
            // Toggle for boolean fields
            if matches!(app.dump_field, DumpField::AllCommits) {
                app.dump_all = !app.dump_all;
                return;
            }
            if matches!(app.dump_field, DumpField::Format) {
                app.dump_format = (app.dump_format + 1) % 2;
            }
        }
        KeyCode::Backspace => match app.dump_field {
            DumpField::Branch => {
                app.dump_branch.pop();
            }
            DumpField::Commit => {
                app.dump_commit.pop();
            }
            DumpField::Output => {
                app.dump_output.pop();
            }
            DumpField::Email => {
                app.dump_email.pop();
            }
            _ => {}
        },
        KeyCode::Char(c) => match app.dump_field {
            DumpField::Branch => app.dump_branch.push(c),
            DumpField::Commit => app.dump_commit.push(c),
            DumpField::Output => app.dump_output.push(c),
            DumpField::Email => app.dump_email.push(c),
            _ => {}
        },
        _ => {}
    }
}

fn handle_result(key: KeyCode, app: &mut App) {
    match key {
        KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') => {
            app.screen = Screen::Menu;
        }
        _ => {}
    }
}

// -- Command executors -------------------------------------------------------
// These suspend the TUI (leave raw mode and alternate screen) so command
// functions can print freely, then re-enter the TUI after completion.

fn suspend_tui() {
    let _ = disable_raw_mode();
    let _ = execute!(stdout(), LeaveAlternateScreen);
}

fn resume_tui() {
    let _ = enable_raw_mode();
    let _ = execute!(
        stdout(),
        EnterAlternateScreen,
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
    );
}

fn execute_start(app: &mut App, branch: &str) {
    suspend_tui();
    match commands::cmd_start(branch) {
        Ok(()) => {
            app.output_log
                .push(format!("[OK] Branch '{}' created.", branch));
        }
        Err(e) => {
            app.output_log.push(format!("[ERROR] {:#}", e));
        }
    }
    resume_tui();
}

fn execute_finish(app: &mut App, title: &str) {
    suspend_tui();
    match commands::cmd_finish(title) {
        Ok(()) => {
            app.output_log
                .push("[OK] Pull Request created.".to_string());
        }
        Err(e) => {
            app.output_log.push(format!("[ERROR] {:#}", e));
        }
    }
    resume_tui();
}

fn execute_dump(app: &mut App) {
    let branch = if app.dump_branch.is_empty() {
        None
    } else {
        Some(app.dump_branch.as_str())
    };
    let commit = if app.dump_commit.is_empty() {
        None
    } else {
        Some(app.dump_commit.as_str())
    };
    let format = app.dump_format_label();
    let output = if app.dump_output.is_empty() {
        "."
    } else {
        &app.dump_output
    };
    let email = if app.dump_email.is_empty() {
        None
    } else {
        Some(app.dump_email.as_str())
    };

    suspend_tui();
    match commands::cmd_dump(branch, commit, app.dump_all, format, output, email) {
        Ok(()) => {
            app.output_log.push("[OK] Dump completed.".to_string());
        }
        Err(e) => {
            app.output_log.push(format!("[ERROR] {:#}", e));
        }
    }
    resume_tui();
}
