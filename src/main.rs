use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use kira::{
    manager::{
        AudioManager, AudioManagerSettings,
        backend::DefaultBackend,
    },
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
};
use std::{
    error::Error,
    sync::mpsc::{self, Receiver},
    time::{Duration, Instant},
    {io, thread},
};
use std::collections::HashMap;
use crossbeam::channel::unbounded;
use invaders::{frame, render};
use invaders::frame::{Drawable, Frame, new_frame};
use invaders::invaders::Invaders;
use invaders::player::Player;

fn main() -> Result <(), Box<dyn Error>> {


    // Audio

    // Create an audio manager. This plays sounds and manages resources.
    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;
    let sound_data = StaticSoundData::from_file("sounds/shoot.mp3", StaticSoundSettings::default())?;

    let mut audios = HashMap::new();
    audios.insert("shoot", StaticSoundData::from_file("sounds/shoot.mp3", StaticSoundSettings::default())?);
    audios.insert("lose", StaticSoundData::from_file("sounds/lose.mp3", StaticSoundSettings::default())?);
    audios.insert("explode", StaticSoundData::from_file("sounds/explosion.mp3", StaticSoundSettings::default())?);
    audios.insert("move", StaticSoundData::from_file("sounds/move.mp3", StaticSoundSettings::default())?);
    audios.insert("startup", StaticSoundData::from_file("sounds/game-start.mp3", StaticSoundSettings::default())?);
    audios.insert("win", StaticSoundData::from_file("sounds/win.mp3", StaticSoundSettings::default())?);

    manager.play(audios.get("startup").unwrap().clone());


    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide);


    // Render loop in a separate thread
    let (render_tx, render_rx) = unbounded();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(frame) => frame,
                Err(_) => break
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();

    // Game Loop 
    'gameloop: loop {
        // Per-frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();
        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {

                    KeyCode::Char('a') =>{
                        player.move_left();
                    }
                    KeyCode::Char('d') =>{
                        player.move_right();
                    }
                    KeyCode::Char(' ') =>{
                        if player.shoot() {
                            manager.play(audios.get("shoot").unwrap().clone());
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        manager.play(audios.get("lose").unwrap().clone());
                        thread::sleep(Duration::from_secs(1));
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }
        // Updates
        player.update(delta);
        if invaders.update(delta) {
            manager.play(audios.get("move").unwrap().clone());
        }

        if player.detect_hits(&mut invaders) {
            manager.play(audios.get("explode").unwrap().clone());
        }



        // Draw and render
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));

        // Win-lose condition
        if invaders.all_killed(){
            manager.play(audios.get("win").unwrap().clone());
            thread::sleep(Duration::from_secs(3));
            break;
        }

        if invaders.reached_bottom(){
            manager.play(audios.get("lose").unwrap().clone());
            thread::sleep(Duration::from_secs(1));
            break;
        }
    }

    // Cleanup

    drop(render_tx); // newer versions of rust don't require it, so I leave just for the sake of clarity
    render_handle.join().unwrap();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
