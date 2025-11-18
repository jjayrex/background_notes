use anyhow::Result;
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::Html,
    routing::{get, post},
};
use rdev::{Event, EventType, Key, listen};
use serde::Serialize;
use std::{
    sync::{Arc, Mutex},
    thread,
};
use tokio::net::TcpListener;

static INDEX_HTML: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/index.html"));

#[derive(Debug, Default)]
struct NotesState {
    recording: bool,
    current_note: String,
    notes: Vec<String>,
}

#[derive(Serialize)]
struct NotesSnapshot {
    recording: bool,
    current_note: String,
    notes: Vec<String>,
}

type SharedState = Arc<Mutex<NotesState>>;

#[tokio::main]
async fn main() -> Result<()> {
    let state: SharedState = Arc::new(Mutex::new(NotesState::default()));

    // Spawn keyboard listener
    {
        let key_state = state.clone();
        thread::spawn(move || {
            if let Err(e) = listen(move |event| handle_key_event(event, &key_state)) {
                eprintln!("Error in keyboard listener: {:?}", e);
            }
        });
    }

    // HTTP server
    let app = Router::new()
        .route("/", get(index))
        .route("/state", get(get_state))
        .route("/clear", post(clear_notes))
        .with_state(state);
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    println!("Listening on 127.0.0.1:7878");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_state(State(state): State<SharedState>) -> Json<NotesSnapshot> {
    let s = state.lock().unwrap();
    let snapshot = NotesSnapshot {
        recording: s.recording,
        current_note: s.current_note.clone(),
        notes: s.notes.clone(),
    };
    Json(snapshot)
}

async fn clear_notes(State(state): State<SharedState>) -> StatusCode {
    let mut s = state.lock().unwrap();
    s.notes.clear();
    StatusCode::NO_CONTENT
}

async fn index() -> Html<&'static str> {
    Html(INDEX_HTML)
}

fn handle_key_event(event: Event, state: &SharedState) {
    if let EventType::KeyPress(key) = event.event_type {
        let mut s = state.lock().unwrap();

        match key {
            // Record/Stop Recording
            Key::F9 => {
                s.recording = !s.recording;
                if !s.recording {
                    let note = s.current_note.clone();

                    if !note.trim().is_empty() {
                        s.notes.push(note);
                    }
                    s.current_note.clear();
                }
            }
            // Cancel current recording
            Key::Escape => {
                s.recording = false;
                s.current_note.clear();
            }
            _ => {
                if !s.recording {
                    return;
                }

                match key {
                    // Basic functions
                    Key::Return => s.current_note.push('\n'),
                    Key::Space => s.current_note.push(' '),
                    Key::Backspace => {
                        s.current_note.pop();
                    }

                    // Arrow keys
                    Key::UpArrow => s.current_note.push('↑'),
                    Key::DownArrow => s.current_note.push('↓'),
                    Key::LeftArrow => s.current_note.push('←'),
                    Key::RightArrow => s.current_note.push('→'),

                    // Regular keys
                    k => {
                        if let Some(ch) = key_to_char(k) {
                            s.current_note.push(ch);
                        }
                    }
                }
            }
        }
    }
}

fn key_to_char(key: Key) -> Option<char> {
    use Key::*;

    Some(match key {
        KeyA => 'a',
        KeyB => 'b',
        KeyC => 'c',
        KeyD => 'd',
        KeyE => 'e',
        KeyF => 'f',
        KeyG => 'g',
        KeyH => 'h',
        KeyI => 'i',
        KeyJ => 'j',
        KeyK => 'k',
        KeyL => 'l',
        KeyM => 'm',
        KeyN => 'n',
        KeyO => 'o',
        KeyP => 'p',
        KeyQ => 'q',
        KeyR => 'r',
        KeyS => 's',
        KeyT => 't',
        KeyU => 'u',
        KeyV => 'v',
        KeyW => 'w',
        KeyX => 'x',
        KeyY => 'y',
        KeyZ => 'z',

        Num0 | Kp0 => '0',
        Num1 | Kp1 => '1',
        Num2 | Kp2 => '2',
        Num3 | Kp3 => '3',
        Num4 | Kp4 => '4',
        Num5 | Kp5 => '5',
        Num6 | Kp6 => '6',
        Num7 | Kp7 => '7',
        Num8 | Kp8 => '8',
        Num9 | Kp9 => '9',

        Minus => '-',
        Equal => '=',
        LeftBracket => '[',
        RightBracket => ']',
        SemiColon => ';',
        Quote => '\'',
        BackQuote => '`',
        BackSlash => '\\',
        Comma => ',',
        Dot => '.',
        Slash => '/',

        _ => return None,
    })
}
