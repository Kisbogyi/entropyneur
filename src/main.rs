use crossterm::ExecutableCommand;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::Marker;
use ratatui::prelude::Style;
use ratatui::widgets::Axis;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::Chart;
use ratatui::widgets::Dataset;
use ratatui::widgets::GraphType;
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::{stdout, Result};
use std::{fs::File, io::Read};

fn calc_entropy() -> std::io::Result<Vec<(f64, f64)>> {
    let chunk_size = 128;
    let filename = "/usr/bin/ls";

    let mut file_input = File::open(filename)?;
    let mut buf = vec![];
    file_input.read_to_end(&mut buf)?;

    let buffers: Vec<&[u8]> = buf.chunks(chunk_size).collect();
    let mut outputs = vec![];
    let mut i = 0.0;
    for buffer in buffers {
        let mut bytes = vec![0; 256];
        for value in buffer.to_vec().into_iter() {
            bytes[usize::from(value)] += 1;
        }

        let summa: u16 = bytes.iter().sum();
        let mut p = bytes
            .iter()
            .map(|&item| f64::from(item) / f64::from(summa))
            .collect::<Vec<_>>();

        p.retain(|&item| item != 0.0);

        let output = p
            .iter()
            .fold(0.0, |entropy, &item| entropy - item * item.log(256.0));

        outputs.push((i, output));
        i += 1.0;
    }
    Ok(outputs)
}

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // TODO main loop
    loop {
        terminal.draw(|frame| {
            let binding = calc_entropy().unwrap();
            let datasets = vec![
                // Line chart
                Dataset::default()
                    .name("data2")
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
                .labels(vec!["0.0".into(), "0.0".into(), "1.0".into()]);

            // Create the Y axis and define its properties
            let y_axis = Axis::default()
                .title("Y Axis".red())
                .style(Style::default().white())
                .bounds([0.0, 1.0])
                .labels(vec!["0.0".into(), "0.0".into(), "1.0".into()]);

            frame.render_widget(
                Chart::new(datasets)
                    .block(Block::new().title("Chart"))
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
