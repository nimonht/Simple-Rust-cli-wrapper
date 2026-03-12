use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use super::app::{App, DumpField, Screen};

/// Render the entire UI.  All colours derive from `Style::default()` so they
/// automatically respect the terminal theme.
pub fn draw_ui(frame: &mut Frame, app: &App) {
    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(1)])
        .split(frame.area());

    draw_body(frame, app, outer[0]);
    draw_status_bar(frame, app, outer[1]);
}

fn draw_body(frame: &mut Frame, app: &App, area: Rect) {
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(22), Constraint::Min(40)])
        .split(area);

    draw_sidebar(frame, app, columns[0]);
    draw_main(frame, app, columns[1]);
}

fn draw_sidebar(frame: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = App::menu_items()
        .iter()
        .enumerate()
        .map(|(i, &label)| {
            let style = if i == app.menu_index {
                Style::default().add_modifier(Modifier::REVERSED | Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(Line::from(Span::styled(format!(" {} ", label), style)))
        })
        .collect();

    let block = Block::default()
        .title(" Actions ")
        .borders(Borders::ALL)
        .border_style(Style::default());

    let list = List::new(items).block(block);
    frame.render_widget(list, area);
}

fn draw_main(frame: &mut Frame, app: &App, area: Rect) {
    match &app.screen {
        Screen::Menu => draw_welcome(frame, area),
        Screen::StartInput => draw_input_screen(frame, app, area, "Start Branch", "Branch name:"),
        Screen::FinishInput => draw_input_screen(frame, app, area, "Finish PR", "PR title:"),
        Screen::DumpConfig => draw_dump_screen(frame, app, area),
        Screen::ResultView => draw_result(frame, app, area),
    }
}

fn draw_welcome(frame: &mut Frame, area: Rect) {
    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            " git-workflow",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(" A CLI tool that automates common Git workflows."),
        Line::from(""),
        Line::from(" Use the arrow keys to navigate, Enter to select."),
        Line::from(""),
        Line::from(" Commands:"),
        Line::from("   Start Branch  - Sync default branch and create a feature branch"),
        Line::from("   Finish PR     - Stage, commit, push, and open a Pull Request"),
        Line::from("   Dump Commits  - Export commits as patch or diff files"),
    ];

    let block = Block::default()
        .title(" git-workflow ")
        .borders(Borders::ALL);
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: false });
    frame.render_widget(paragraph, area);
}

fn draw_input_screen(frame: &mut Frame, app: &App, area: Rect, title: &str, label: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(5), Constraint::Min(3)])
        .split(area);

    // Input area
    let input_text = vec![
        Line::from(""),
        Line::from(format!(" {}", label)),
        Line::from(format!(" > {}_", app.input)),
    ];

    let block = Block::default()
        .title(format!(" {} ", title))
        .borders(Borders::ALL);
    let paragraph = Paragraph::new(input_text)
        .block(block)
        .wrap(Wrap { trim: false });
    frame.render_widget(paragraph, chunks[0]);

    // Output area
    draw_output_log(frame, app, chunks[1]);
}

fn draw_dump_screen(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(12), Constraint::Min(3)])
        .split(area);

    let field_lines: Vec<Line> = vec![
        Line::from(""),
        dump_field_line(
            "Branch (empty = current):",
            &app.dump_branch,
            &app.dump_field,
            DumpField::Branch,
        ),
        dump_field_line(
            "Commit SHA (optional):",
            &app.dump_commit,
            &app.dump_field,
            DumpField::Commit,
        ),
        dump_toggle_line(
            "All commits:",
            app.dump_all,
            &app.dump_field,
            DumpField::AllCommits,
        ),
        dump_toggle_line_str(
            "Format:",
            app.dump_format_label(),
            &app.dump_field,
            DumpField::Format,
        ),
        dump_field_line(
            "Output path:",
            &app.dump_output,
            &app.dump_field,
            DumpField::Output,
        ),
        dump_field_line(
            "Email (optional):",
            &app.dump_email,
            &app.dump_field,
            DumpField::Email,
        ),
        Line::from(""),
        {
            let style = if app.dump_field == DumpField::Execute {
                Style::default().add_modifier(Modifier::REVERSED | Modifier::BOLD)
            } else {
                Style::default().add_modifier(Modifier::BOLD)
            };
            Line::from(Span::styled("   [ Execute ]", style))
        },
    ];

    let block = Block::default()
        .title(" Dump Commits ")
        .borders(Borders::ALL);
    let paragraph = Paragraph::new(field_lines)
        .block(block)
        .wrap(Wrap { trim: false });
    frame.render_widget(paragraph, chunks[0]);

    draw_output_log(frame, app, chunks[1]);
}

fn dump_field_line<'a>(
    label: &'a str,
    value: &'a str,
    current: &DumpField,
    field: DumpField,
) -> Line<'a> {
    let cursor = if *current == field { " > " } else { "   " };
    let style = if *current == field {
        Style::default().add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    let display_value = if *current == field {
        format!("{}{} {}_", cursor, label, value)
    } else {
        format!("{}{} {}", cursor, label, value)
    };
    Line::from(Span::styled(display_value, style))
}

fn dump_toggle_line<'a>(
    label: &'a str,
    value: bool,
    current: &DumpField,
    field: DumpField,
) -> Line<'a> {
    let cursor = if *current == field { " > " } else { "   " };
    let style = if *current == field {
        Style::default().add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    let display = if value { "yes" } else { "no" };
    Line::from(Span::styled(
        format!("{}{} [{}]  (Enter to toggle)", cursor, label, display),
        style,
    ))
}

fn dump_toggle_line_str<'a>(
    label: &'a str,
    value: &'a str,
    current: &DumpField,
    field: DumpField,
) -> Line<'a> {
    let cursor = if *current == field { " > " } else { "   " };
    let style = if *current == field {
        Style::default().add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    Line::from(Span::styled(
        format!("{}{} [{}]  (Enter to toggle)", cursor, label, value),
        style,
    ))
}

fn draw_result(frame: &mut Frame, app: &App, area: Rect) {
    let lines: Vec<Line> = app
        .output_log
        .iter()
        .map(|s| Line::from(s.as_str()))
        .collect();

    let block = Block::default().title(" Result ").borders(Borders::ALL);
    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });
    frame.render_widget(paragraph, area);
}

fn draw_output_log(frame: &mut Frame, app: &App, area: Rect) {
    let lines: Vec<Line> = app
        .output_log
        .iter()
        .map(|s| Line::from(format!(" {}", s)))
        .collect();

    let block = Block::default().title(" Output ").borders(Borders::ALL);
    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });
    frame.render_widget(paragraph, area);
}

fn draw_status_bar(frame: &mut Frame, app: &App, area: Rect) {
    let help = match &app.screen {
        Screen::Menu => " q: quit | j/k: navigate | Enter: select",
        Screen::StartInput | Screen::FinishInput => " Esc: back | Enter: run",
        Screen::DumpConfig => " Esc: back | Tab/arrows: navigate fields | Enter: toggle/run",
        Screen::ResultView => " Esc/Enter: back to menu",
    };

    let style = Style::default().add_modifier(Modifier::REVERSED);
    let paragraph = Paragraph::new(Line::from(Span::styled(help, style)));
    frame.render_widget(paragraph, area);
}
