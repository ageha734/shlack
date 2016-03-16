## Shlack

A command line utility to pipe text into Slack messages.

Some cool use cases
```sh
$ echo "anyone wanna grab lunch? :thumbsup:" | shlack -c random
```
```sh
$ gist < file.json | shlack -c luke
```
```sh
$ hub pull-request | shlack -c general
```

For now, you need to clone the repo and have Rust installed to use this.

Set an environment variable `SLACK_TOKEN`. You can get a token here: https://api.slack.com/docs/oauth-test-tokens

Clone the repo and run this:

```sh
$ cargo build
$ echo hi | target/debug/shlack -v
```

### Commands:
```
-v --verbose
-c --channel
-p --prepend
-a --append
```

### TODO:
  - attachments
  - brew install
