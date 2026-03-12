/// Application state for the TUI.

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Screen {
    Menu,
    StartInput,
    FinishInput,
    DumpConfig,
    ResultView,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DumpField {
    Branch,
    Commit,
    AllCommits,
    Format,
    Output,
    Email,
    Execute,
}

impl DumpField {
    pub fn next(&self) -> Self {
        match self {
            Self::Branch => Self::Commit,
            Self::Commit => Self::AllCommits,
            Self::AllCommits => Self::Format,
            Self::Format => Self::Output,
            Self::Output => Self::Email,
            Self::Email => Self::Execute,
            Self::Execute => Self::Branch,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Self::Branch => Self::Execute,
            Self::Commit => Self::Branch,
            Self::AllCommits => Self::Commit,
            Self::Format => Self::AllCommits,
            Self::Output => Self::Format,
            Self::Email => Self::Output,
            Self::Execute => Self::Email,
        }
    }
}

pub struct App {
    pub screen: Screen,
    pub menu_index: usize,
    pub input: String,
    pub output_log: Vec<String>,
    pub should_quit: bool,

    // Dump configuration
    pub dump_branch: String,
    pub dump_commit: String,
    pub dump_all: bool,
    pub dump_format: usize, // 0 = patch, 1 = diff
    pub dump_output: String,
    pub dump_email: String,
    pub dump_field: DumpField,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: Screen::Menu,
            menu_index: 0,
            input: String::new(),
            output_log: vec!["Ready. Select an action to get started.".to_string()],
            should_quit: false,
            dump_branch: String::new(),
            dump_commit: String::new(),
            dump_all: true,
            dump_format: 0,
            dump_output: ".".to_string(),
            dump_email: String::new(),
            dump_field: DumpField::Branch,
        }
    }

    pub fn menu_items() -> &'static [&'static str] {
        &["Start Branch", "Finish PR", "Dump Commits", "Quit"]
    }

    pub fn dump_format_label(&self) -> &str {
        match self.dump_format {
            0 => "patch",
            1 => "diff",
            _ => "patch",
        }
    }

    pub fn reset_dump(&mut self) {
        self.dump_branch.clear();
        self.dump_commit.clear();
        self.dump_all = true;
        self.dump_format = 0;
        self.dump_output = ".".to_string();
        self.dump_email.clear();
        self.dump_field = DumpField::Branch;
    }
}
