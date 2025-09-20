//! Minimal core skeleton for Luna Assistant.
//! Provides a simple event bus and plugin trait so future skills can be registered.

use std::fmt;

/// Basic event structure carrying a name and string payload.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Event {
    pub name: String,
    pub payload: String,
}

impl Event {
    /// Convenience constructor.
    pub fn new<N: Into<String>, P: Into<String>>(name: N, payload: P) -> Self {
        Self {
            name: name.into(),
            payload: payload.into(),
        }
    }
}

/// Errors that can occur while handling events.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventError {
    pub message: String,
}

impl EventError {
    pub fn new<M: Into<String>>(message: M) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for EventError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub type EventResult = Result<(), EventError>;

/// Trait implemented by plugins (skills) interested in receiving events.
pub trait Plugin: Send + Sync {
    /// Name of the plugin. Used for diagnostics and filtering.
    fn name(&self) -> &str;

    /// Called whenever an event is dispatched on the bus.
    fn handle(&self, event: &Event) -> EventResult;

    /// Optional filter. By default, the plugin receives every event.
    fn accepts(&self, _event: &Event) -> bool {
        true
    }
}

/// Very small event bus capable of registering plugins and dispatching events.
#[derive(Default)]
pub struct EventBus {
    plugins: Vec<Box<dyn Plugin>>,
}

impl EventBus {
    /// Create an empty bus.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a plugin on the bus.
    pub fn register<P: Plugin + 'static>(&mut self, plugin: P) {
        self.plugins.push(Box::new(plugin));
    }

    /// Dispatch an event to all plugins that accept it.
    pub fn dispatch(&self, event: &Event) -> Vec<EventResult> {
        self.plugins
            .iter()
            .filter(|plugin| plugin.accepts(event))
            .map(|plugin| plugin.handle(event))
            .collect()
    }

    /// List the names of registered plugins.
    pub fn plugin_names(&self) -> Vec<String> {
        self.plugins.iter().map(|p| p.name().to_string()).collect()
    }
}

/// Simple echo plugin used as reference implementation.
pub struct EchoPlugin;

impl Plugin for EchoPlugin {
    fn name(&self) -> &str {
        "echo"
    }

    fn handle(&self, event: &Event) -> EventResult {
        if event.name == "echo" {
            Ok(())
        } else {
            Err(EventError::new("Echo plugin received unexpected event"))
        }
    }

    fn accepts(&self, event: &Event) -> bool {
        event.name == "echo"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    struct RecordingPlugin {
        name: String,
        received: Arc<Mutex<Vec<Event>>>,
    }

    impl RecordingPlugin {
        fn new(name: &str, received: Arc<Mutex<Vec<Event>>>) -> Self {
            Self {
                name: name.to_string(),
                received,
            }
        }
    }

    impl Plugin for RecordingPlugin {
        fn name(&self) -> &str {
            &self.name
        }

        fn handle(&self, event: &Event) -> EventResult {
            self.received.lock().unwrap().push(event.clone());
            Ok(())
        }
    }

    #[test]
    fn registers_plugins_and_dispatches_events() {
        let recorder = Arc::new(Mutex::new(Vec::new()));
        let plugin = RecordingPlugin::new("recorder", recorder.clone());

        let mut bus = EventBus::new();
        bus.register(plugin);
        assert_eq!(bus.plugin_names(), vec!["recorder".to_string()]);

        let event = Event::new("any", "payload");
        let results = bus.dispatch(&event);
        assert_eq!(results.len(), 1);
        assert!(results[0].is_ok());

        let stored = recorder.lock().unwrap();
        assert_eq!(stored.len(), 1);
        assert_eq!(stored[0], event);
    }

    #[test]
    fn echo_plugin_accepts_only_echo_events() {
        let plugin = EchoPlugin;
        let mut bus = EventBus::new();
        bus.register(plugin);

        let echo = Event::new("echo", "hello");
        let other = Event::new("other", "ignored");

        let echo_results = bus.dispatch(&echo);
        assert_eq!(echo_results.len(), 1);
        assert!(echo_results[0].is_ok());

        let other_results = bus.dispatch(&other);
        assert_eq!(other_results.len(), 1);
        assert!(other_results[0].is_err());
    }
}
