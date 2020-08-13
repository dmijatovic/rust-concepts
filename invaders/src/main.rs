use std::error::Error;
use rusty_audio::Audio;
use std::io;
use std::io::{Stdout};
use std::time::{Duration};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event, terminal,ExecutableCommand};
use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent};

fn add_audio_files(a: &mut Audio){
  a.add("explode", "sound/explode.wav");
  a.add("pew", "sound/pew.wav");
}

fn open_terminal()->Stdout{
  let mut screen = io::stdout();

  let r = terminal::enable_raw_mode();
  if r.is_err(){
    panic!("Failed to enable raw mode");
  }
  // create new window on the top of existingone
  let r = screen.execute(EnterAlternateScreen);
  if r.is_err(){
    panic!("Failed to enter alternate screen");
  }
  let r = screen.execute(cursor::Hide);
  if r.is_err(){
    panic!("Failed to hide cursor");
  }
  screen
}

fn close_terminal(screen: &mut Stdout){
  let r = screen.execute(cursor::Show);
  if r.is_err(){
    panic!("Failed to hide cursor");
  }
  let r = screen.execute(LeaveAlternateScreen);
  if r.is_err(){
    panic!("Failed to hide cursor");
  }
  let r = terminal::disable_raw_mode();
  if r.is_err(){
    panic!("Failed to hide cursor");
  }
}

fn play_game(){
  // no modifier key
  let _no_modifiers = event::KeyModifiers::empty();
  'gameloop: loop{
    //get input
    match read().unwrap() {
      //i think this speaks for itself
      Event::Key(KeyEvent {
          code: event::KeyCode::Char('q'),
          modifiers: _no_modifiers,
          //clearing the screen
      }) => break 'gameloop,
      Event::Key(event::KeyEvent {
          code: KeyCode::Char('t'),
          modifiers: _no_modifiers,
      }) => println!("You dit alt + t"),
      _ => (),
    }
    // Input
    // while let (event::poll(Duration::default())) = true{
    //   let key_event = event::read();
    //   if let Event::Key(key_event) = {
    //     match key_event.code {
    //       // KeyCode::Left => player.move_left(),
    //       // KeyCode::Right => player.move_right(),
    //       // KeyCode::Char(' ') | KeyCode::Enter => {
    //       //     if player.shoot() {
    //       //         audio.play("pew");
    //       //     }
    //       // }
    //       KeyCode::Esc | KeyCode::Char('q') => {
    //           // audio.play("lose");
    //           break 'gameloop;
    //       },
    //       _ => {}
    //     }
    //   }
    // }
  }
}


fn main() -> Result<(),Box<dyn Error>> {
  // println!("Hello, world!");
  let mut audio = Audio::new();
  // add audios
  add_audio_files(&mut audio);
  // create new terminal
  let mut terminal = open_terminal();
  // play one
  audio.play("explode");
  //cleanup
  audio.wait();

  play_game();

  //close terminal
  close_terminal(&mut terminal);
  // close main with Ok
  Ok(())
}
