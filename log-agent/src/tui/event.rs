use crossterm::event::{self, KeyEvent, MouseEvent, Event as CrosstermEvent};
use std::time::Duration;
use tokio::sync::mpsc;

/// Événements de l'application
#[derive(Debug)]
pub enum Event {
    /// Événement clavier
    Key(KeyEvent),
    /// Événement souris
    Mouse(MouseEvent),
    /// Tick périodique (pour countdown, refresh)
    Tick,
    /// Redimensionnement du terminal (width, height)
    #[allow(dead_code)]
    Resize(u16, u16),
}

/// Gestionnaire d'événements async
pub struct EventHandler {
    rx: mpsc::UnboundedReceiver<Event>,
    #[allow(dead_code)]
    tx: mpsc::UnboundedSender<Event>,
}

impl EventHandler {
    /// Crée un nouveau gestionnaire avec un tick rate donné
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let event_tx = tx.clone();

        // Spawn la tâche de poll des événements
        tokio::spawn(async move {
            let mut tick_interval = tokio::time::interval(tick_rate);
            
            loop {
                let event = tokio::select! {
                    _ = tick_interval.tick() => {
                        Event::Tick
                    }
                    result = poll_crossterm_event() => {
                        match result {
                            Some(evt) => evt,
                            None => continue,
                        }
                    }
                };

                if event_tx.send(event).is_err() {
                    // Le receiver a été droppé, on arrête
                    break;
                }
            }
        });

        Self { rx, tx }
    }

    /// Retourne le prochain événement
    pub async fn next(&mut self) -> Option<Event> {
        self.rx.recv().await
    }
}

/// Poll les événements crossterm de manière async
async fn poll_crossterm_event() -> Option<Event> {
    // On utilise tokio::task::spawn_blocking car event::poll est bloquant
    tokio::task::spawn_blocking(|| -> Option<Event> {
        // Poll avec un timeout court pour ne pas bloquer trop longtemps
        if event::poll(Duration::from_millis(50)).ok()? {
            match event::read().ok()? {
                CrosstermEvent::Key(key) => Some(Event::Key(key)),
                CrosstermEvent::Mouse(mouse) => Some(Event::Mouse(mouse)),
                CrosstermEvent::Resize(w, h) => Some(Event::Resize(w, h)),
                _ => None,
            }
        } else {
            None
        }
    })
    .await
    .ok()
    .flatten()
}
