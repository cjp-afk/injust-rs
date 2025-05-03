use crate::ui::styles;
use crate::winapi::types::Process;
use ratatui::widgets::{Block, Borders};
use ratatui::{
    prelude::*,
    widgets::{List, ListItem, ListState},
};
use styles::{Theme, CYAN_THEME};

/// Immutable-until-selection application state.
pub struct App {
    theme: Theme,
    processes: Vec<Process>,
    state: ListState,
}

impl App {
    pub fn new(processes: Vec<Process>) -> Self {
        let mut state = ListState::default();
        if !processes.is_empty() {
            state.select(Some(0));
        }
        Self {
            theme: CYAN_THEME,
            processes,
            state,
        }
    }

    /* ---------------- navigation ---------------- */
    pub fn next(&mut self) {
        let i = self
            .state
            .selected()
            .map(|i| (i + 1) % self.processes.len())
            .unwrap_or(0);
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let len = self.processes.len();
        let i = self
            .state
            .selected()
            .map(|i| if i == 0 { len - 1 } else { i - 1 })
            .unwrap_or(0);
        self.state.select(Some(i));
    }

    pub fn selected(&self) -> Option<&Process> {
        self.state.selected().map(|i| &self.processes[i])
    }

    /* ---------------- rendering ---------------- */
    pub fn ui(&mut self, f: &mut Frame) {
        let t = self.theme; // local alias

        /* --------- build list items ---------- */
        let items: Vec<ListItem> = self
            .processes
            .iter()
            .map(|p| {
                ListItem::new(format!("{:<6}  {}", p.pid, p.title))
                    .style(Style::default().fg(t.base_fg))
            })
            .collect();

        /* --------- widget styling ------------ */
        let list = List::new(items)
            .block(
                Block::default()
                    .title(" Processes ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(t.accent)),
            )
            .style(Style::default().fg(t.base_fg)) // default list text
            .highlight_symbol("❯ ") // ← plain &str now
            .highlight_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(t.accent)
                    .add_modifier(Modifier::BOLD),
            );

        f.render_stateful_widget(list, f.area(), &mut self.state);
    }
}
