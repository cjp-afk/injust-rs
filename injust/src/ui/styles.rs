use ratatui::style::Color;

#[derive(Clone, Copy)]
pub struct Theme {
    pub accent: Color,
    pub base_fg: Color,
}

pub const CYAN_THEME: Theme = Theme {
    accent: Color::Rgb(0x00, 0xd7, 0xff), // bright-cyan (≈ #00D7FF)
    base_fg: Color::Reset,
};
