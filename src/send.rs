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
}

pub fn send(msg: Msg) -> Result<(), String> {
    let mut client = slack::RtmClient::new(&msg.token);
    try!(client.login().map_err(|_| "unable to login to slack".to_string()));
    let channel_addr = try!(find_channel_addr(&client, msg.channel.clone()));
    try!(client
         .post_message(&channel_addr, &msg.text.clone(), None)
         .map_err(|_| "unable to send message".to_string()));
    Ok(())
}

fn find_channel_addr(client: &slack::RtmClient, channel: String)
        -> Result<String, String> {
    client
        .get_channels()
        .iter()
        .find(|c| c.name.to_string() == channel)
        .map(|r| r.id.clone())
        .or(client
             .get_users()
             .iter()
             .find(|u| u.name.to_string() == channel)
             .map(|r| r.id.clone()))
         .or(client
             .get_groups()
             .iter()
             .find(|g| g.name.to_string() == channel)
             .map(|r| r.id.clone()))
         .ok_or("unable to find channel".to_string())
}
