use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use image::GenericImageView;
use ratatui::{
    prelude::*,
    widgets::{
        canvas::{Canvas, Points},
        *,
    },
};

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of target file
    #[arg(short, long)]
    file_path: String,
}

use std::io::{self, stdout};
fn main() -> io::Result<()> {
    let args = Args::parse();

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    terminal.draw(|frame| ui(frame, &args.file_path))?;

    while !handle_events()? {}

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame, file_path: &str) {
    let img = image::open(file_path).expect("File cou");
    let (img_width, img_height) = img.dimensions();

    let canvas = Canvas::default()
        .marker(Marker::HalfBlock)
        .block(
            Block::new()
                .title(frame.size().to_string())
                .borders(Borders::ALL),
        )
        .x_bounds([0.0, img_width as f64])
        .y_bounds([0.0, img_height as f64])
        .paint(|ctx| {
            for y in 0..img_height {
                for x in 0..img_width {
                    let pixel = img.get_pixel(x, img_height - y - 1);
                    let color = Color::Rgb(pixel[0], pixel[1], pixel[2]);
                    ctx.draw(&Points {
                        coords: &[(x as f64, y as f64)],
                        color,
                    });
                }
            }
        });
    frame.render_widget(canvas, frame.size());
}
