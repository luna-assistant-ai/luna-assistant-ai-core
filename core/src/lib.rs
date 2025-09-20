//! Minimal core skeleton for Luna Assistant.
//! Provides an asynchronous event bus and plugin trait so future skills can be registered.

use std::{fmt, sync::Arc, time::Duration};

use async_trait::async_trait;

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
#[async_trait]
pub trait Plugin: Send + Sync {
    /// Name of the plugin. Used for diagnostics and filtering.
    fn name(&self) -> &str;

    /// Optional filter. By default, the plugin receives every event.
    fn accepts(&self, _event: &Event) -> bool {
        true
    }

    /// Synchronous handler for CPU-bound operations.
    fn handle_sync(&self, _event: &Event) -> EventResult {
        Err(EventError::new("Sync handler not implemented"))
    }

    /// Async handler. Default wraps sync via spawn_blocking.
    async fn handle_async(&self, event: Arc<Event>) -> EventResult {
        self.handle_sync(&event)
    }
}

/// Very small event bus capable of registering plugins and dispatching events.
pub struct EventBus {
    plugins: Vec<Arc<dyn Plugin>>, 
    timeout: Duration,
}

impl EventBus {
    /// Create a bus with default timeout (5s).
    pub fn new() -> Self {
        Self::with_timeout(Duration::from_secs(5))
    }

    /// Create a bus with custom timeout duration.
    pub fn with_timeout(timeout: Duration) -> Self {
        Self {
            plugins: Vec::new(),
            timeout,
        }
    }

    /// Register a plugin on the bus.
    pub fn register<P: Plugin + 'static>(&mut self, plugin: P) {
        self.plugins.push(Arc::new(plugin));
    }

    /// Dispatch an event to all plugins that accept it.
    pub async fn dispatch(&self, event: &Event) -> Vec<EventResult> {
        use futures::future::join_all;

        let shared_event = Arc::new(event.clone());
        let timeout = self.timeout;

        let handles: Vec<_> = self
            .plugins
            .iter()
            .filter(|plugin| plugin.accepts(event))
            .map(|plugin| {
                let plugin = Arc::clone(plugin);
                let event = Arc::clone(&shared_event);
                tokio::spawn(async move {
                    match tokio::time::timeout(timeout, plugin.handle_async(event)).await {
                        Ok(result) => result,
                        Err(_) => Err(EventError::new("Plugin timed out")),
                    }
                })
            })
            .collect();

        join_all(handles)
            .await
            .into_iter()
            .map(|result| match result {
                Ok(inner) => inner,
                Err(join_error) => Err(EventError::new(format!("Plugin task panicked: {}", join_error))),
            })
            .collect()
    }

    /// List the names of registered plugins.
    pub fn plugin_names(&self) -> Vec<String> {
        self.plugins.iter().map(|p| p.name().to_string()).collect()
    }
}

/// Simple echo plugin used as reference implementation.
pub struct EchoPlugin;

#[async_trait]
impl Plugin for EchoPlugin {
    fn name(&self) -> &str {
        "echo"
    }

    fn handle_sync(&self, event: &Event) -> EventResult {
        if event.name == "echo" {
            Ok(())
        } else {
            Err(EventError::new("Echo plugin received unexpected event"))
        }
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

    #[async_trait]
    impl Plugin for RecordingPlugin {
        fn name(&self) -> &str {
            &self.name
        }

        fn handle_sync(&self, event: &Event) -> EventResult {
            self.received.lock().unwrap().push(event.clone());
            Ok(())
        }
    }

    impl RecordingPlugin {
        fn new(name: &str, received: Arc<Mutex<Vec<Event>>>) -> Self {
            Self {
                name: name.to_string(),
                received,
            }
        }
    }

    #[tokio::test]
    async fn registers_plugins_and_dispatches_events() {
        let recorder = Arc::new(Mutex::new(Vec::new()));
        let plugin = RecordingPlugin::new("recorder", recorder.clone());

        let mut bus = EventBus::new();
        bus.register(plugin);
        assert_eq!(bus.plugin_names(), vec!["recorder".to_string()]);

        let event = Event::new("any", "payload");
        let results = bus.dispatch(&event).await;
        assert_eq!(results.len(), 1);
        assert!(results[0].is_ok());

        let stored = recorder.lock().unwrap();
        assert_eq!(stored.len(), 1);
        assert_eq!(stored[0], event);
    }

    #[tokio::test]
    async fn echo_plugin_accepts_only_echo_events() {
        let plugin = EchoPlugin;
        let mut bus = EventBus::new();
        bus.register(plugin);

        let echo = Event::new("echo", "hello");
        let other = Event::new("other", "ignored");

        let echo_results = bus.dispatch(&echo).await;
        assert_eq!(echo_results.len(), 1);
        assert!(echo_results[0].is_ok());

        let other_results = bus.dispatch(&other).await;
        assert_eq!(other_results.len(), 1);
        assert!(other_results[0].is_err());
    }

    struct SlowPlugin;

    #[async_trait]
    impl Plugin for SlowPlugin {
        fn name(&self) -> &str {
            "slow"
        }

        async fn handle_async(&self, _event: Arc<Event>) -> EventResult {
            tokio::time::sleep(Duration::from_millis(100)).await;
            Ok(())
        }
    }

    #[tokio::test]
    async fn applies_timeout_to_plugins() {
        let mut bus = EventBus::with_timeout(Duration::from_millis(10));
        bus.register(SlowPlugin);

        let event = Event::new("any", "payload");
        let results = bus.dispatch(&event).await;
        assert_eq!(results.len(), 1);
        assert!(results[0].is_err());
        assert_eq!(results[0].as_ref().err().unwrap().message, "Plugin timed out");
    }
}
