use anyhow::Result;
use rdev::{listen, Event, EventType, Key};
use std::sync::mpsc::{channel, Sender};
use std::thread;
// use std::time::Duration;
use shared::logging;
use std::sync::Arc;

pub struct HotkeyManager {
    sender: Sender<Event>,
    active_hotkeys: Vec<(Vec<Key>, Key, Arc<dyn Fn() + Send + Sync>)>,
}

impl HotkeyManager {
    pub fn new() -> Result<Self> {
        let (sender, _receiver) = channel();

        let manager = HotkeyManager {
            sender: sender.clone(),
            active_hotkeys: Vec::new(),
        };

        // Start the event listener thread
        thread::spawn(move || {
            if let Err(e) = listen(move |event| {
                if let Err(e) = sender.send(event) {
                    logging::error(&format!("Error sending hotkey event: {:?}", e));
                }
            }) {
                logging::error(&format!("Error in hotkey listener: {:?}", e));
            }
        });

        Ok(manager)
    }

    #[allow(dead_code)]
    pub fn check_hotkey(&self, event: &Event, modifiers: &[Key], key: Key) -> bool {
        if event.event_type != EventType::KeyPress(key) {
            return false;
        }

        // Check if all modifiers are pressed
        modifiers.iter().all(|modifier| {
            event.name == Some(format!("{:?}", modifier))
        })
    }

    pub fn start_listening(&mut self, modifiers: &[Key], key: Key, callback: impl Fn() + Send + Sync + 'static) {
        logging::info(&format!(
            "Registering hotkey: {:?} + {:?}",
            modifiers,
            key
        ));

        // Store the callback in an Arc for thread-safe reference counting
        let callback = Arc::new(callback);
        let callback_clone = callback.clone();

        self.active_hotkeys.push((
            modifiers.to_vec(),
            key,
            callback,
        ));

        let _sender = self.sender.clone();

        // Create clones for the thread
        let modifiers_clone = modifiers.to_vec();
        let key_clone = key;

        thread::spawn(move || {
            let (_, receiver) = channel::<Event>();
            for event in receiver.iter() {
                if Self::check_hotkey_static(&event, &modifiers_clone, key_clone) {
                    logging::debug(&format!(
                        "Hotkey triggered: {:?} + {:?}",
                        modifiers_clone,
                        key_clone
                    ));
                    // Execute the callback
                    (callback_clone)();
                }
            }
        });
    }

    fn check_hotkey_static(event: &Event, modifiers: &[Key], key: Key) -> bool {
        if event.event_type != EventType::KeyPress(key) {
            return false;
        }

        modifiers.iter().all(|modifier| {
            event.name == Some(format!("{:?}", modifier))
        })
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
