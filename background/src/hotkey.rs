use anyhow::Result;
use rdev::{listen, Event, Key, EventType::*};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use shared::logging;

pub struct HotkeyManager {
    sender: Sender<Event>,
    receiver: Arc<Mutex<Receiver<Event>>>,
    active_hotkeys: Vec<(Vec<Key>, Key, Arc<dyn Fn() + Send + Sync>)>,
    pressed_keys: Arc<Mutex<HashSet<Key>>>,
}

impl HotkeyManager {
    pub fn new() -> Result<Self> {
        let (sender, receiver) = channel();

        let manager = HotkeyManager {
            sender: sender.clone(),
            receiver: Arc::new(Mutex::new(receiver)),
            active_hotkeys: Vec::new(),
            pressed_keys: Arc::new(Mutex::new(HashSet::new())),
        };

        Ok(manager)
    }

    pub fn check_hotkey(&self, event: &Event, modifiers: &[Key], key: Key) -> bool {
        match event.event_type {
            KeyPress(k) if k == key => {
                // Check if all modifiers are currently pressed
                let pressed_keys = self.pressed_keys.lock().unwrap();
                modifiers.iter().all(|m| pressed_keys.contains(m))
            },
            _ => false,
        }
    }

    pub fn start_listening(&mut self, modifiers: &[Key], key: Key, callback: impl Fn() + Send + Sync + 'static) {
        logging::info(&format!(
            "Registering hotkey: {:?} + {:?}",
            modifiers,
            key
        ));

        // Store the callback in an Arc for thread-safe reference counting
        let callback = Arc::new(callback);
        
        self.active_hotkeys.push((
            modifiers.to_vec(),
            key,
            callback.clone(),
        ));

        let sender = self.sender.clone();
        let pressed_keys = self.pressed_keys.clone();
        
        // Clone modifiers to extend their lifetime
        let modifiers = modifiers.to_vec();
        
        // Start the listener in a thread
        thread::spawn(move || {
            if let Err(e) = listen(move |event| {
                // Track key state
                match event.event_type {
                    KeyPress(k) => {
                        let mut keys = pressed_keys.lock().unwrap();
                        keys.insert(k);
                    },
                    KeyRelease(k) => {
                        let mut keys = pressed_keys.lock().unwrap();
                        keys.remove(&k);
                    },
                    _ => {}
                }
                
                // Check if our hotkey combination is pressed
                match event.event_type {
                    KeyPress(k) if k == key => {
                        let keys = pressed_keys.lock().unwrap();
                        let all_modifiers_pressed = modifiers.iter().all(|m| keys.contains(m));
                        
                        if all_modifiers_pressed {
                            logging::info("Hotkey triggered!");
                            callback();
                        }
                    },
                    _ => {}
                }
                
                // Forward to channel
                let _ = sender.send(event);
            }) {
                logging::error(&format!("Error in hotkey listener: {:?}", e));
            }
        });
    }

    #[allow(dead_code)]
    pub fn remove_hotkey(&mut self, modifiers: &[Key], key: Key) -> Result<()> {
        logging::info(&format!(
            "Removing hotkey: {:?} + {:?}",
            modifiers,
            key
        ));

        self.active_hotkeys.retain(|(m, k, _)| {
            m != modifiers || *k != key
        });

        Ok(())
    }

    #[allow(dead_code)]
    pub fn clear_hotkeys(&mut self) {
        logging::info("Clearing all hotkeys");
        self.active_hotkeys.clear();
    }
}
