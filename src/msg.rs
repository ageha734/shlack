use slack;

pub struct Msg {
    text: String,
    channel: String,
    token: String,
}

impl Msg {
    pub fn new(text: String, channel: String, token: String) -> Msg {
        Msg {
            text: text,
            channel: channel,
            token: token,
        }
    }

    pub fn send(&self) -> Result<(), String> {
        let mut client = slack::RtmClient::new(&self.token);
        try!(client
             .login()
             .map_err(|_| "unable to login to slack".to_string()));
        let addr = try!(self.find_addr(client.get_channels())
                        .or(self.find_addr(client.get_users()))
                        .or(self.find_addr(client.get_groups()))
                        .ok_or("unable to find channel".to_string()));
        try!(client
             .post_message(&addr, &self.text.clone(), None)
             .map_err(|_| "unable to send message".to_string()));
        Ok(())
    }

    fn find_addr<T: HasName + HasId>(&self, recipients: Vec<T>)
            -> Option<String> {
        recipients
            .iter()
            .find(|x| x.name() == self.channel)
            .map(|x| x.id())
    }
}

trait HasId {
    fn id(&self) -> String;
}

impl HasId for slack::Channel {
    fn id(&self) -> String {
        self.id.to_string()
    }
}
impl HasId for slack::User {
    fn id(&self) -> String {
        self.id.to_string()
    }
}

impl HasId for slack::Group {
    fn id(&self) -> String {
        self.id.to_string()
    }
}

trait HasName {
    fn name(&self) -> String;
}

impl HasName for slack::Channel {
    fn name(&self) -> String {
        self.name.to_string()
    }
}

impl HasName for slack::User {
    fn name(&self) -> String {
        self.name.to_string()
    }
}

impl HasName for slack::Group {
    fn name(&self) -> String {
        self.name.to_string()
    }
}
