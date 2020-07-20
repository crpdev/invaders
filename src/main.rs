use std::error::Error;
// use rusty_audio::Audio;
use std::{io, thread};
use crossterm::{terminal, ExecutableCommand, event};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor::{Hide, Show};
use std::time::{Duration, Instant};
use crossterm::event::{KeyCode, Event};
use std::sync::mpsc;
use invaders::{frame, render};
use invaders::player::Player;
use invaders::frame::Drawable;

fn main() -> Result<(), Box<dyn Error>> {

    // Setup Audio for the game
    /*
    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("start", "start.wav");
    audio.add("win", "win.wav");

    // Play the Game Start Audio
    // Audio is played on a different thread
    audio.play("start");
    */

    // Setting up the Terminal to play the game
    // The game window is the OS Terminal, so we get from stdout
    let mut stdout = io::stdout();

    // To read keyboard input use crossterm's terminal and usage of (?) will panic in case of errors
    terminal::enable_raw_mode()?;
    // Similar to vim, the game runs on an alternate screen rather than the command window
    stdout.execute(EnterAlternateScreen)?;
    // Hide the cursor on the game window
    stdout.execute(Hide)?;

    // Render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv(){
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    // Create a new player on the alternate terminal
    let mut player = Player::new();
    let mut instant = Instant::now();

    // Gameloop

    'gameloop: loop {

        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_fame = frame::new_frame();

        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            // audio.play("pew");
                        }
                    },
                    KeyCode::Esc | KeyCode::Char('q') => {
                        // audio.play("lose");
                        break 'gameloop
                    },
                    _ => {}
                }
            }
            
        }

    //     Updates

        player.update(delta);

    //     Draw and render
        player.draw(&mut curr_fame);
        let _ = render_tx.send(curr_fame);
        thread::sleep(Duration::from_millis(1));
    }


    // Cleanup Section
    // Wait for the audio playback thread to complete
    drop(render_tx);
    render_handle.join().unwrap();
    // audio.wait();

    // Fallback to the command window
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    // Return the OK result
    Ok(())
}
