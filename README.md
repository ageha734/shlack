## Shlack

A command line utility to pipe text into Slack messages.

```sh
$ echo "anyone wanna grab lunch? :thumbsup:" | shlack random
```
```sh
$ gist < file.json | shlack luke
```
```sh
$ hub pull-request | shlack general
```

### Install:
Set an environment variable `SLACK_TOKEN`. You can get a token here: https://api.slack.com/docs/oauth-test-tokens
```sh
$ cargo build
$ echo hi | target/debug/shlack
```
This sends a message to Slackbot.

### Commands:
```
-v --verbose
-p --prepend
-a --append
```
