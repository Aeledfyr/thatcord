# thatcord

Prototype Rust Discord library supporting tokio 0.2, futures 0.3 and async/.await.

Heavy emphasis on **prototype**, as it currently can only listen, not send.
And only supports two events at that.

## Installation

While I have a crate named `thatcord` on `crates.io`, it's currently empty, as
I want to get something stable for an initial release.

In the meantime, get it from this repo like so:

```toml
[dependencies]
thatcord = { git = "https://github.com/Admicos/thatcord" }
```

## Documentation

Currently very lacking. However, the `examples/` directory should be enough for
the current full features of this library.

Also, there is some docstrings here and there, but they'll probably need
some formatting, so no fancy HTML docs yet.

## Code Quality

Horrible. Actually, no. Saying this is horrible would be insulting other horrible
projects.

If you know a thing or two about Rust, please contribute!

## License

**Apache License 2.0 or MIT License** depending on your preference.
