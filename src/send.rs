use slack;

pub struct Msg {
    pub text: String,
    pub channel: String,
}

pub fn send(msg: Msg) -> Result<(), String> {
    let token = "xoxp-11118560705-11114969687-17074931648-82542c337c";
    let mut client = slack::RtmClient::new(token);
    client.login();

    match find_recipient(&client, &msg) {
        Some(rec) => {
            if let Err(error) = client.post_message(
                &format!("#{}", rec), &msg.text.clone(), None) {
                    println!("{}", error);
                };
            Ok(())
        },
        None => Err("channel does not exist".to_string()),
    }
}

fn find_recipient(client: &slack::RtmClient, msg: &Msg) -> Option<String> {
    client
        .get_channels()
        .iter()
        .find(|channel| channel.name == msg.channel.to_owned())
        .map(|channel| channel.name.clone())
}
