RSSt
=====================

[RSSt](https://github.com/quinoa42/rsst) is a commandline tool that dump articles in followed feeds into offline files when run. It aims to be the tool for RSS what [OfflineIMAP](https://github.com/OfflineIMAP/offlineimap) has been for IMAP.

Usage
---------------------

See `rsst --help` for all options, but generally it's this workflow:

1. Create a config file in `XDG_CONFIG_HOME/rsst/config.toml`:

```toml
[setting]
output_format = "html"
output_dir = "~/documents/rsst/"

[source]
example = "https://example.com/rss.xml"
```

If `output_dir` is not given, the default one is "~/rsst". Sources listed in `source` section are the followed feeds, where `example` is the alias (used as the subdirectory name) and `"https://example.com/rss.xml"` is the feed file address.

2. Simply run `rsst` everytime you want to check if there are new articles. RSSt will keep track of the last newest articles in `XDG_DATA_HOME/rsst`, and incrementally retrieving new articles next time. You can sort files based on created/modified time to see what's new.

3. Retrieved articles will be in the given `output_dir` or `~/rsst`. You can read them or parse them with whatever the way you want (web browser, for example).

Installation / Compilation
---------------------

Install it with whatever ways you like, for example:

```bash
https://github.com/quinoa42/rsst.git
cd rsst
cargo build --release
```

Then compiled execrable will be located at `target/release/rsst`.

License
---------------------

MIT
