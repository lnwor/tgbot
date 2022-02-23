# TGbot

This is a very simple telegram bot meant to easily upload videos from command
line.

## Installation

Clone the repo:
`git clone https://github.com/lnwor/tgbot && cd tgbot`

Install the package:
`cargo install --path .`

Create the configuration file with your token here: `~/.config/tgbot.yml`.

An example of configuration file can be found at the root of the repo.

```yaml
token: "abcdefghijklmnopqrstuvwxyz123456789"
```

## Usage

After you set up your configuration file, to use the bot you just have to run
the following command:
`tgbot <chat_id> /path/to/file "caption to add to the file"`

### Limits

- The telegram Bot API has a limit of 50MB per file.
- After 1 minute of upload time the connection times out.
