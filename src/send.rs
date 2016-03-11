use slack::RtmClient;

pub struct Msg {
    pub text: String,
    pub channel: String,
}

pub struct Sender {
    client: RtmClient,
}

impl Sender {
    pub fn new(client: RtmClient) -> Sender {
        Sender { client: client }
    }

    pub fn send(&self, msg: Msg) -> Result<(), String> {
        match self.find_channel_id(&msg) {
            Some(channel_id) => {
                self.client.send_message(&channel_id, &msg.text.clone());
                Ok(())
            },
            None => Err("channel does not exist".to_string())
        }
    }

    fn find_channel_id(&self, msg: &Msg) -> Option<String> {
        self.client
            .get_channels()
            .iter()
            .find(|channel| channel.name == msg.channel.to_owned())
            .map(|channel| channel.id.clone())
    }
}
