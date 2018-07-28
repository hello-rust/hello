# 👋 hello

A little command-line tool which helps reduce manual work for content creators.  

### Why?

Social media is hard. On one side, you would like to keep all platforms updated
and share your creations with the world. On the other hand, maintaining all of
them takes away a substantial amount of time from actually creating content.

You might say that this is not an important problem to solve - and I agree to
some extent. But having a side-project next to work is a tough thing to handle,
and every little step that can be automated saves some time for more important
things like... uhm... friends and family?

You might say that this is cheating - and you might be right. Submitting an
article is a soulless work of honor that should be done manually as a sign of
respect (yo) to your audience. But guess what: most of your audience doesn't
care about your link on social media, it cares about your content.

You might say there are similar tools - and that's true. There's
[Buffer](https://buffer.com), which can do everything that this tool can, but
better. Nevertheless, the goal is to build a free tool under a permissive
license that is easy to hack on.


Therefore I created **hello**.

### Installation

```
cargo install
```

### Usage

Create a `.env` file with your credentials. See `.env_dist` for an example.

After that, run `hello -h` to get started.

For example, to submit a link to Reddit, run
```
hello reddit subredditnamehere "Give up" "https://www.youtube.com/watch?v=DLzxrzFCyOs"
```

### Currently supported platforms

* [X] Reddit
* [X] Twitter
* [X] HackerNews
* [ ] Patreon (tough one, as the API does not support publishing yet)
* [ ] Discourse (tough one, as only admins can get an API key)
* [ ] Bring your own!

### How to get your credentials

##### Reddit

Create an app at reddit.com/prefs/apps.
After that, add your credentials to `.env`.
Then you can run `hello` like so:

```
reddit yoursubredditname "I gave up" "https://www.youtube.com/watch?v=DLzxrzFCyOs"
```

##### Twitter

Make an app for yourself at apps.twitter.com. On first execution of `hello`,
you will get an access token via OAuth.

Then run the following command to send a tweet:

```
hello twitter "Hello! https://github.com/hello-rust/hello"
```

Follow the instructions on the screen to save that for all subsequent requests.

#### HackerNews

Since HackerNews doesn't have an API for submitting links, we have to be
creative.
We use the awesome [fantoccini](https://github.com/jonhoo/fantoccini) to control
a [WebDriver](https://github.com/Fyrd/caniuse/issues/2757#issuecomment-304529217)
compatible browser.

1. Install geckodriver by fetching the [latest build from their release page](https://github.com/mozilla/geckodriver/releases).
2. Add your HN credentials to `.env`.

Example:

```
hello hn "Show HN: Hello, a CLI tool for managing social media" https://github.com/hello-rust/hello
```

### Contributing

This tool was made possible by the excellent patrons of ["Hello
Rust!"](https://github.com/hello-rust/show), a show about the Rust programming
language. With their help, this tool is made available to the public under dual
MIT/Apache license.

[Become a patron](https://www.patreon.com/bePatron?c=1568097) now to support
future work and send pull requests for supporting other platforms.
