mod core;
mod ui;
mod winapi;

use ui::app::App;
use winapi::winsafe::safe_enum_windows;

use color_eyre::eyre::Result;
use ratatui::crossterm::event::KeyEventKind;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let processes = safe_enum_windows()?; // requires your uploaded winsafe.rs
    if processes.is_empty() {
        println!("No visible processes discovered.");
        return Ok(());
    }

    // Terminal bootstrap (alternate screen + raw mode)
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    let mut app = App::new(processes);

    // ---------- main event loop ----------
    loop {
        terminal.draw(|f| app.ui(f))?;

        if event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Up => app.previous(),
                        KeyCode::Down => app.next(),
                        KeyCode::Enter | KeyCode::Esc | KeyCode::Char('q') => break,
                        _ => {}
                    }
                }
            }
        }
    }

    // Terminal teardown
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Surface the chosen process (could cascade into further automation)
    if let Some(proc) = app.selected() {
        println!("Chosen ➜  {}  (pid {})", proc.title, proc.pid);
    }

    Ok(())
}
