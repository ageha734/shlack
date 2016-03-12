use slack;

pub fn send(text: &str, channel: &str) -> Result<(), String> {
    let token = "";
    let mut client = slack::RtmClient::new(token);
    try!(client.login().map_err(|_| "unable to login to slack".to_string()));

    if channel_exists(&client, channel) {
        try!(client
             .post_message(&format!("#{}", channel), &text, None)
             .map_err(|_| "unable to send message".to_string()));
        Ok(())
    } else {
       Err("channel does not exist".to_string())
    }
}

fn channel_exists(client: &slack::RtmClient, channel: &str) -> bool {
    client
        .get_channels()
        .iter()
        .any(|c| c.name.to_string() == channel.to_string())
}
