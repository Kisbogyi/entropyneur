use crossterm::ExecutableCommand;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::Marker;
use ratatui::prelude::Style;
use ratatui::prelude::{CrosstermBackend, Stylize, Terminal};
use ratatui::widgets::Axis;
use ratatui::widgets::Block;
use ratatui::widgets::Chart;
use ratatui::widgets::Dataset;
use ratatui::widgets::GraphType;
use std::env;
use std::io::{stdout, Result};

mod entropy;

fn main() -> Result<()> {
    let argument: Vec<_> = env::args().collect();
    if argument.len() <= 1 {
        println!("Please provide file!");
        return Ok(());
    }
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    loop {
        terminal.draw(|frame| {
            let binding = entropy::calc_entropy(&argument[1], 128).expect("File name was expected");
            let datasets = vec![
                // Line chart
                Dataset::default()
                    .name(format!("{}", argument[1]))
                    .marker(Marker::Braille)
                    .graph_type(GraphType::Line)
                    .style(Style::default().magenta())
                    .data(&binding),
            ];

            // Create the X axis and define its properties
            let x_axis = Axis::default()
                .title("X Axis".red())
                .style(Style::default().white())
                .bounds([0.0, binding.len() as f64])
                .labels(vec![
                    "0.0".into(),
                    format!("{}", binding.len() / 2).into(),
                    format!("{}", binding.len()).into(),
                ]);

            // Create the Y axis and define its properties
            let y_axis = Axis::default()
                .title("Y Axis".red())
                .style(Style::default().white())
                .bounds([0.0, 1.0])
                .labels(vec!["0.0".into(), "0.5".into(), "1.0".into()]);

            // Render Chart
            frame.render_widget(
                Chart::new(datasets)
                    .block(Block::new().title("Entropy"))
                    .x_axis(x_axis)
                    .y_axis(y_axis),
                frame.size(),
            )
        })?;

        // Handle q to exit
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
