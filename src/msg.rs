//! Message handling module for Slack communication.

use slack;

/// Represents a Slack message with text, channel, and authentication token.
///
/// # Examples
///
/// ```rust
/// use shlack::Msg;
///
/// let msg = Msg::new(
///     "Hello, World!".to_string(),
///     "general".to_string(),
///     "xoxb-your-token-here".to_string(),
/// );
///
/// assert_eq!(msg.text(), "Hello, World!");
/// assert_eq!(msg.channel(), "general");
/// ```
pub struct Msg {
    text: String,
    channel: String,
    token: String,
}

impl Msg {
    /// Creates a new Slack message.
    ///
    /// # Arguments
    ///
    /// * `text` - The message content to send
    /// * `channel` - The target channel name (e.g., "general", "random")
    /// * `token` - The Slack bot token for authentication
    ///
    /// # Examples
    ///
    /// ```rust
    /// use shlack::Msg;
    ///
    /// let msg = Msg::new(
    ///     "Hello, World!".to_string(),
    ///     "general".to_string(),
    ///     "xoxb-your-token-here".to_string(),
    /// );
    /// ```
    pub fn new(text: String, channel: String, token: String) -> Msg {
        Msg {
            text: text,
            channel: channel,
            token: token,
        }
    }

    /// Returns the message text.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use shlack::Msg;
    ///
    /// let msg = Msg::new("Hello".to_string(), "general".to_string(), "token".to_string());
    /// assert_eq!(msg.text(), "Hello");
    /// ```
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns the target channel name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use shlack::Msg;
    ///
    /// let msg = Msg::new("Hello".to_string(), "general".to_string(), "token".to_string());
    /// assert_eq!(msg.channel(), "general");
    /// ```
    pub fn channel(&self) -> &str {
        &self.channel
    }

    /// Returns the Slack token.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use shlack::Msg;
    ///
    /// let msg = Msg::new("Hello".to_string(), "general".to_string(), "token".to_string());
    /// assert_eq!(msg.token(), "token");
    /// ```
    pub fn token(&self) -> &str {
        &self.token
    }

    /// Sends the message to Slack.
    ///
    /// This method connects to Slack using the RTM API, finds the target channel,
    /// and sends the message.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the message was sent successfully
    /// * `Err(String)` - If there was an error during login, channel lookup, or sending
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - Unable to login to Slack (invalid token)
    /// - Unable to find the specified channel
    /// - Unable to send the message
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use shlack::Msg;
    ///
    /// let msg = Msg::new(
    ///     "Hello, World!".to_string(),
    ///     "general".to_string(),
    ///     "xoxb-your-valid-token-here".to_string(),
    /// );
    ///
    /// match msg.send() {
    ///     Ok(()) => println!("Message sent successfully!"),
    ///     Err(e) => eprintln!("Failed to send message: {}", e),
    /// }
    /// ```
    pub fn send(&self) -> Result<(), String> {
        let client = slack::RtmClient::login(&self.token)
            .map_err(|_| "unable to login to slack".to_string())?;

        let start_response = client.start_response();

        let addr = self.find_addr(start_response.channels.as_ref().unwrap_or(&vec![]))
                        .or_else(|| self.find_addr(start_response.users.as_ref().unwrap_or(&vec![])))
                        .or_else(|| self.find_addr(start_response.groups.as_ref().unwrap_or(&vec![])))
                        .ok_or("unable to find channel".to_string())?;

        client.sender()
             .send_message(&addr, &self.text)
             .map_err(|_| "unable to send message".to_string())?;
        Ok(())
    }

    /// Finds the Slack ID for a channel, user, or group by name.
    ///
    /// This method searches through a list of recipients (channels, users, or groups)
    /// to find one that matches the target channel name.
    ///
    /// # Arguments
    ///
    /// * `recipients` - A slice of items that implement both `HasName` and `HasId` traits
    ///
    /// # Returns
    ///
    /// * `Some(String)` - The Slack ID if a matching recipient is found
    /// * `None` - If no matching recipient is found
    ///
    /// # Examples
    ///
    /// ```rust
    /// use shlack::{Msg, HasId, HasName};
    ///
    /// struct MockChannel {
    ///     id: String,
    ///     name: String,
    /// }
    ///
    /// impl HasId for MockChannel {
    ///     fn id(&self) -> String {
    ///         self.id.clone()
    ///     }
    /// }
    ///
    /// impl HasName for MockChannel {
    ///     fn name(&self) -> String {
    ///         self.name.clone()
    ///     }
    /// }
    ///
    /// let msg = Msg::new("test".to_string(), "general".to_string(), "token".to_string());
    /// let channels = vec![
    ///     MockChannel { id: "C123456".to_string(), name: "general".to_string() },
    /// ];
    ///
    /// let result = msg.find_addr(&channels);
    /// assert_eq!(result, Some("C123456".to_string()));
    /// ```
    pub fn find_addr<T: HasName + HasId>(&self, recipients: &[T]) -> Option<String> {
        recipients
            .iter()
            .find(|x| x.name() == self.channel)
            .map(|x| x.id())
    }
}

/// Trait for objects that have a Slack ID.
///
/// This trait is implemented by Slack entities like channels, users, and groups
/// to provide access to their unique identifiers.
pub trait HasId {
    /// Returns the Slack ID of the entity.
    ///
    /// # Returns
    ///
    /// A `String` containing the Slack ID (e.g., "C1234567890" for channels,
    /// "U1234567890" for users, "G1234567890" for groups).
    fn id(&self) -> String;
}

impl HasId for slack::Channel {
    fn id(&self) -> String {
        self.id.clone().unwrap_or_default()
    }
}
impl HasId for slack::User {
    fn id(&self) -> String {
        self.id.clone().unwrap_or_default()
    }
}

impl HasId for slack::Group {
    fn id(&self) -> String {
        self.id.clone().unwrap_or_default()
    }
}

/// Trait for objects that have a name.
///
/// This trait is implemented by Slack entities like channels, users, and groups
/// to provide access to their human-readable names.
pub trait HasName {
    /// Returns the name of the entity.
    ///
    /// # Returns
    ///
    /// A `String` containing the name (e.g., "general" for channels,
    /// "john.doe" for users, "private-group" for groups).
    fn name(&self) -> String;
}

impl HasName for slack::Channel {
    fn name(&self) -> String {
        self.name.clone().unwrap_or_default()
    }
}

impl HasName for slack::User {
    fn name(&self) -> String {
        self.name.clone().unwrap_or_default()
    }
}

impl HasName for slack::Group {
    fn name(&self) -> String {
        self.name.clone().unwrap_or_default()
    }
}
