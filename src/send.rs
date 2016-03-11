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

    pub fn send(&mut self, msg: Msg) -> Result<(), String> {
        self.client.login();

        match self.find_channel_id(&msg) {
            Some(chan) => {
                if let Err(error) =
                    self.client.post_message(
                        &format!("#{}", chan), &msg.text.clone(), None) {
                        println!("{}", error);
                    };
                Ok(())
            },
            None => {
                Err("channel does not exist".to_string())
            }
        }
    }

    fn find_channel_id(&self, msg: &Msg) -> Option<String> {
        self.client
            .get_channels()
            .iter()
            .find(|channel| channel.name == msg.channel.to_owned())
            .map(|channel| channel.name.clone())
    }
}
