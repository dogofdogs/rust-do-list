use std::{
    io::{self},
    thread,
    time::Duration,
};

use crossterm::{
    event::{self, read, Event, KeyEventKind, KeyEventState, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    prelude::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use tasks::tasks::tasks;
mod tasks;

////fn main() {
//    println!("1 for seeing tasks");
//    println!("2 for adding a task");
//    println!("3 for deleting a task");:#![warn()]
//    let ree = input_question(input_number());
//    println!("{}", ree);
////}

fn main() -> Result<(), io::Error> {
    //set up the terminal
    enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear().ok();
    terminal.draw(|f| {
        thread::sleep(Duration::from_secs(1));
        let size = f.size();
        let block = Block::default().title("test").borders(Borders::ALL);
        //creates a textblock widget for later use
        let textblock = Paragraph::new(tasks()).block(Block::default().borders(Borders::ALL));
        f.render_widget(block, size);
        f.render_widget(textblock, size)
    })?;
    pause();
    disable_raw_mode()?;
    Ok(())
}

fn pause() {
    loop {
        if let Ok(event) = read() {
            if let Event::Key(event::KeyEvent {
                code: event::KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            }) = event
            {
                break;
            };
        }
    }
}

fn input_number() -> i32 {
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer);
    for x in buffer.trim().chars() {
        if x.is_numeric() == false {
            return "999".parse::<i32>().unwrap();
        } else {
        }
    }
    return buffer.trim().parse::<i32>().unwrap();
}

fn input_question(number: i32) -> String {
    match number {
        1 => "views all tasks".to_string(),
        2 => "creates a task".to_string(),
        3 => "deletes a task".to_string(),
        999 => "please dont put in charcters but numbers".to_string(),
        _ => "pick one of the options".to_string(),
    }
}
