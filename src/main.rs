use anyhow::{Context, Result};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, ScrollbarState},
};
use std::{io, process::Command, time::Duration, io::{BufRead, BufReader}, sync::mpsc};
use std::thread;

struct App {
    input_file: String,
    start_time: String,
    end_time: String,
    output_file: String,
    current_focus: Focus,
    message: String,
    is_processing: bool,
    scroll_state: ScrollbarState,
    scroll_offset: usize,
}

#[derive(PartialEq)]
enum Focus {
    InputFile,
    StartTime,
    EndTime,
    OutputFile,
}

impl App {
    fn new() -> App {
        App {
            input_file: String::new(),
            start_time: String::from("00:00:00"),
            end_time: String::new(),
            output_file: String::new(),
            current_focus: Focus::InputFile,
            message: String::from("Enter input file path"),
            is_processing: false,
            scroll_state: ScrollbarState::default(),
            scroll_offset: 0,
        }
    }

    fn process_input(&mut self, key: KeyCode) {
        if self.is_processing {
            return; // Ignore input while processing
        }

        match key {
            KeyCode::Tab => {
                self.current_focus = match self.current_focus {
                    Focus::InputFile => {
                        self.message = String::from("Enter start time (HH:MM:SS)");
                        Focus::StartTime
                    }
                    Focus::StartTime => {
                        self.message = String::from("Enter end time (HH:MM:SS)");
                        Focus::EndTime
                    }
                    Focus::EndTime => {
                        self.message = String::from("Enter output file path");
                        Focus::OutputFile
                    }
                    Focus::OutputFile => {
                        self.message = String::from("Enter input file path");
                        Focus::InputFile
                    }
                };
            }
            KeyCode::Char(c) => {
                let current_string = match self.current_focus {
                    Focus::InputFile => &mut self.input_file,
                    Focus::StartTime => &mut self.start_time,
                    Focus::EndTime => &mut self.end_time,
                    Focus::OutputFile => &mut self.output_file,
                };
                current_string.push(c);
            }
            KeyCode::Backspace => {
                let current_string = match self.current_focus {
                    Focus::InputFile => &mut self.input_file,
                    Focus::StartTime => &mut self.start_time,
                    Focus::EndTime => &mut self.end_time,
                    Focus::OutputFile => &mut self.output_file,
                };
                current_string.pop();
            }
            KeyCode::Enter => {
                if !self.input_file.is_empty()
                    && !self.start_time.is_empty()
                    && !self.end_time.is_empty()
                    && !self.output_file.is_empty()
                {
                    self.is_processing = true;
                    self.message.clear();
                    self.message.push_str("Starting FFmpeg process...\n");
                    match self.create_clip() {
                        Ok(_) => {
                            self.message.push_str("\nProcess completed.");
                        }
                        Err(e) => {
                            self.message.push_str(&format!("\nError: {}", e));
                        }
                    }
                    self.is_processing = false;
                }
            }
            _ => {}
        }
    }

    fn create_clip(&mut self) -> Result<()> {
        let mut child = Command::new("ffmpeg")
            .args([
                "-i",
                &self.input_file,
                "-ss",
                &self.start_time,
                "-to",
                &self.end_time,
                "-c",
                "copy",
                "-progress",
                "pipe:1",
                "-nostats",
                &self.output_file,
            ])
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .context("Failed to spawn ffmpeg command")?;

        let stderr = child.stderr.take().expect("Failed to capture stderr");
        let (tx, rx) = mpsc::channel();

        // Spawn a thread to read stderr
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if let Ok(line) = line {
                    if tx.send(line).is_err() {
                        break;
                    }
                }
            }
        });

        // Read output lines while the process is running
        while child.try_wait().context("Failed to check process status")?.is_none() {
            while let Ok(line) = rx.try_recv() {
                self.message.push_str(&line);
                self.message.push('\n');
                // Auto-scroll to bottom when new content is added
                self.scroll_offset = self.message.lines().count().saturating_sub(1);
            }
            thread::sleep(Duration::from_millis(50));
        }

        // Read any remaining output
        while let Ok(line) = rx.try_recv() {
            self.message.push_str(&line);
            self.message.push('\n');
            // Auto-scroll to bottom when new content is added
            self.scroll_offset = self.message.lines().count().saturating_sub(1);
        }

        let status = child.wait().context("Failed to wait for ffmpeg")?;

        if status.success() {
            self.message.push_str("\nClip created successfully!");
        } else {
            self.message.push_str("\nError: FFmpeg process failed");
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    // Terminal initialization
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        // Poll more frequently when processing
        let timeout = if app.is_processing {
            Duration::from_millis(50)
        } else {
            Duration::from_millis(100)
        };

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') && key.modifiers == event::KeyModifiers::CONTROL && !app.is_processing {
                    return Ok(());
                }
                app.process_input(key.code);
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),  // Input file
                Constraint::Length(3),  // Start time
                Constraint::Length(3),  // End time
                Constraint::Length(3),  // Output file
                Constraint::Min(10),    // Message area (expanded)
            ]
            .as_ref(),
        )
        .split(f.size());

    let input_block = Block::default()
        .title("Input File")
        .borders(Borders::ALL)
        .border_style(if app.current_focus == Focus::InputFile {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });
    f.render_widget(
        Paragraph::new(app.input_file.as_str()).block(input_block),
        chunks[0],
    );

    let start_block = Block::default()
        .title("Start Time (HH:MM:SS)")
        .borders(Borders::ALL)
        .border_style(if app.current_focus == Focus::StartTime {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });
    f.render_widget(
        Paragraph::new(app.start_time.as_str()).block(start_block),
        chunks[1],
    );

    let end_block = Block::default()
        .title("End Time (HH:MM:SS)")
        .borders(Borders::ALL)
        .border_style(if app.current_focus == Focus::EndTime {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });
    f.render_widget(
        Paragraph::new(app.end_time.as_str()).block(end_block),
        chunks[2],
    );

    let output_block = Block::default()
        .title("Output File")
        .borders(Borders::ALL)
        .border_style(if app.current_focus == Focus::OutputFile {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });
    f.render_widget(
        Paragraph::new(app.output_file.as_str()).block(output_block),
        chunks[3],
    );

    let message_block = Block::default()
        .title("FFmpeg Output")
        .borders(Borders::ALL);

    let mut scroll_state = app.scroll_state.clone();
    let paragraph = Paragraph::new(app.message.as_str())
        .block(message_block)
        .wrap(ratatui::widgets::Wrap { trim: true })
        .scroll((app.scroll_offset as u16, 0));

    f.render_widget(paragraph, chunks[4]);

    let scrollbar = ratatui::widgets::Scrollbar::default()
        .orientation(ratatui::widgets::ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));

    f.render_stateful_widget(
        scrollbar,
        chunks[4].inner(&Margin { vertical: 1, horizontal: 0 }),
        &mut scroll_state,
    );
}
