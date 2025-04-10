use anyhow::{Context, Result};
use rdev::{listen, Event, EventType, Key};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::time::Duration;
use shared::logging;

#[derive(Debug)]
pub struct HotkeyManager {
    sender: Sender<Event>,
    active_hotkeys: Vec<(Vec<Key>, Key, Box<dyn Fn() + Send + 'static>)>,
}

impl HotkeyManager {
    pub fn new() -> Result<Self> {
        let (sender, receiver) = channel();
        
        let manager = HotkeyManager {
            sender,
            active_hotkeys: Vec::new(),
        };
        
        // Start the event listener thread
        thread::spawn(move || {
            if let Err(e) = listen(move |event| {
                if let Err(e) = manager.sender.send(event) {
                    logging::error(&format!("Error sending hotkey event: {}", e));
                }
            }) {
                logging::error(&format!("Error in hotkey listener: {}", e));
            }
        });
        
        Ok(manager)
    }

    pub fn check_hotkey(&self, event: &Event, modifiers: &[Key], key: Key) -> bool {
        if event.event_type != EventType::KeyPress {
            return false;
        }

        // Check if the main key is pressed
        if event.key != Some(key) {
            return false;
        }

        // Check if all modifiers are pressed
        modifiers.iter().all(|modifier| {
            event.modifiers.contains(modifier)
        })
    }

    pub fn start_listening(&mut self, modifiers: &[Key], key: Key, callback: impl Fn() + Send + 'static) {
        logging::info(&format!(
            "Registering hotkey: {:?} + {}",
            modifiers,
            key
        ));
        
        self.active_hotkeys.push((
            modifiers.to_vec(),
            key,
            Box::new(callback),
        ));
        
        let receiver = self.sender.try_clone().unwrap();
        let hotkeys = self.active_hotkeys.clone();
        
        thread::spawn(move || {
            for event in receiver {
                for (modifiers, key, callback) in &hotkeys {
                    if Self::check_hotkey_static(&event, modifiers, *key) {
                        logging::debug(&format!(
                            "Hotkey triggered: {:?} + {}",
                            modifiers,
                            key
                        ));
                        callback();
                    }
                }
            }
        });
    }

    fn check_hotkey_static(event: &Event, modifiers: &[Key], key: Key) -> bool {
        if event.event_type != EventType::KeyPress {
            return false;
        }

        if event.key != Some(key) {
            return false;
        }

        modifiers.iter().all(|modifier| {
            event.modifiers.contains(modifier)
        })
    }

    pub fn remove_hotkey(&mut self, modifiers: &[Key], key: Key) -> Result<()> {
        logging::info(&format!(
            "Removing hotkey: {:?} + {}",
            modifiers,
            key
        ));
        
        self.active_hotkeys.retain(|(m, k, _)| {
            m != modifiers || *k != key
        });
        
        Ok(())
    }

    pub fn clear_hotkeys(&mut self) {
        logging::info("Clearing all hotkeys");
        self.active_hotkeys.clear();
    }
} 