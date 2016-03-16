## Shlack

A command line utility to pipe text into Slack messages.

```sh
$ echo "anyone wanna grab lunch? :thumbsup:" | shlack -c random
```
```sh
$ gist < file.json | shlack -c luke
```
```sh
$ hub pull-request | shlack -c general
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
-c --channel
-p --prepend
-a --append
```
