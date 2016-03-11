use slack::RtmClient;

pub struct Msg {
    text: String,
    send_to: Option<String>,
    prepend_text: Option<String>,
    append_text: Option<String>,
}

impl Msg {
    fn final_msg(&self) -> String {
        format!("{}{}{}",
                self.prepend_text.clone().unwrap_or("".to_string()),
                self.text.clone(),
                self.append_text.clone().unwrap_or("".to_string()))
    }
}

struct Sender {
    client: RtmClient,
}

impl Sender {
    pub fn send(&self, msg: Msg) -> Result<(), String> {
        match self.find_channel_id(&msg) {
            Some(channel_id) => {
                self.client.send_message(&channel_id, &msg.final_msg());
                Ok(())
            },
            None => Err("channel does not exist".to_string())
        }
    }

    fn find_channel_id(&self, msg: &Msg) -> Option<String> {
        if let Some(ref to) = msg.send_to {
            self.client
                .get_channels()
                .iter()
                .find(|channel| channel.name == to.to_owned())
                .map(|channel| channel.id.clone())
        } else {
            None
        }
    }
}
