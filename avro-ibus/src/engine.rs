use avro_core::parser::EnglishToBangla;
use std::sync::Mutex;
use zbus::{interface, zvariant};

pub struct AvroEngine {
    parser: EnglishToBangla,
    buffer: Mutex<String>,
}

impl AvroEngine {
    pub fn new() -> Self {
        let mut parser = EnglishToBangla::new();
        parser.auto_correct = false; // standard for generic usage unless specified
        Self {
            parser,
            buffer: Mutex::new(String::new()),
        }
    }
}

// Ensure zvariant is available to macro
use zbus::zvariant::Value;

#[interface(name = "org.freedesktop.IBus.Engine")]
impl AvroEngine {
    async fn process_key_event(
        &self,
        keyval: u32,
        _keycode: u32,
        state: u32,
        #[zbus(signal_context)] ctxt: zbus::SignalContext<'_>
    ) -> bool {
        // IBus state flags: 1 << 30 indicates KeyRelease.
        if (state & (1 << 30)) != 0 {
            return false;
        }

        // Pass through if a modifier like Ctrl/Alt/Super is active
        if (state & 4) != 0 || (state & 8) != 0 || (state & 64) != 0 {
            return false;
        }

        // To avoid Send bounds issues across await points with MutexGuard
        let (action, transliterated, count) = {
            let mut buffer = self.buffer.lock().unwrap();

            match keyval {
                0xFF0D | 0x0020 => {
                    // Return (0xFF0D) or Space (0x0020) -> commit the current composition
                    if !buffer.is_empty() {
                        let translit = self.parser.convert(&buffer).unwrap_or_default();
                        buffer.clear();
                        (1, translit, 0)
                    } else {
                        (0, String::new(), 0)
                    }
                }
                0xFF08 => {
                    // Backspace
                    if !buffer.is_empty() {
                        buffer.pop();
                        if buffer.is_empty() {
                            (2, String::new(), 0)
                        } else {
                            let translit = self.parser.convert(&buffer).unwrap_or_default();
                            (3, translit, buffer.chars().count())
                        }
                    } else {
                        (0, String::new(), 0)
                    }
                }
                0xFF1B => {
                    // Escape
                    buffer.clear();
                    (2, String::new(), 0)
                }
                _ => {
                    // Printable ASCII
                    if keyval >= 0x21 && keyval <= 0x7E {
                        if let Some(c) = char::from_u32(keyval) {
                            buffer.push(c);
                            let translit = self.parser.convert(&buffer).unwrap_or_default();
                            (4, translit, buffer.chars().count())
                        } else {
                            (0, String::new(), 0)
                        }
                    } else {
                        (0, String::new(), 0)
                    }
                }
            }
        };

        match action {
            1 => {
                let val = zvariant::Value::from(transliterated);
                let _ = Self::commit_text(&ctxt, &val).await;
                let _ = Self::hide_preedit_text(&ctxt).await;
                return true;
            }
            2 => {
                let _ = Self::hide_preedit_text(&ctxt).await;
                // If it was backspace (action 2 with empty string, return true to consume)
                if keyval == 0xFF08 {
                    return true;
                }
                return false;
            }
            3 | 4 => {
                let val = zvariant::Value::from(transliterated);
                let _ = Self::update_preedit_text(&ctxt, &val, count as u32, true).await;
                return true;
            }
            _ => {
                return false;
            }
        }
    }

    async fn enable(&self) {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.clear();
    }

    async fn disable(&self) {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.clear();
    }

    async fn focus_in(&self) {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.clear();
    }

    async fn focus_out(&self) {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.clear();
    }

    async fn reset(&self) {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.clear();
    }

    #[zbus(signal)]
    async fn commit_text(ctxt: &zbus::SignalContext<'_>, text: &Value<'_>) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn update_preedit_text(
        ctxt: &zbus::SignalContext<'_>,
        text: &Value<'_>,
        cursor_pos: u32,
        visible: bool,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn hide_preedit_text(ctxt: &zbus::SignalContext<'_>) -> zbus::Result<()>;
}
