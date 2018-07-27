# Submit

![logo](submit.svg)

A little command-line tool which helps reduce manual work for content creators.
Why?

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


Therefore I created **submit**.

### Installation

```
cargo install
```

### Usage

Create a `.env` file with your credentials. See `.env_dist` for an example.

After that, run `submit -h` to get started.

For example, to submit a link to Reddit, run
```
submit reddit subredditnamehere "Give up" "https://www.youtube.com/watch?v=DLzxrzFCyOs"
```

### Currently supported platforms

* [X] Reddit
* [X] Twitter
* [ ] Patreon (tough one, as the API does not support publishing yet)
* [ ] Discourse (tough one, as the only admins can get an API key)
* [ ] HackerNews (tough one, as there is no API)
* [ ] Bring your own!

### How to get your credentials

##### Reddit

Create an app at reddit.com/prefs/apps.

##### Twitter

Make an app for yourself at apps.twitter.com. On first execution of `submit`,
you will get an access token via OAuth. Follow the instructions on the screen to
save that for all subsequent requests.

### Contributing

This tool was made possible by the excellent patrons of ["Hello
Rust!"](https://github.com/hello-rust/show), a show about the Rust programming
language. With their help, this tool is made available to the public under dual
MIT/Apache license.

[Become a patron](https://www.patreon.com/bePatron?c=1568097) now to support
future work and send pull requests for supporting other platforms.

### Credits

Logo made by [Google](https://www.flaticon.com/authors/google) under Ceative
Commons BY 3.0.
