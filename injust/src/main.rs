#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console in release
mod ui;
mod winapi;

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    Ok(())
}
