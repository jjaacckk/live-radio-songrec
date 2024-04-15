Rust program to listen to live radio streams and recognize currently playing songs using Shazam. 

This project relies heavily on [SongRec](https://github.com/marin-m/SongRec/tree/master), which does not seem to expose many of the necessary functions (meaning I cannot use it as a library via `use SongRec::fingerprinting::*` or similar).