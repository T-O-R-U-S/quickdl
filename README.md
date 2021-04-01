# quickdl
A 100% Rust client to download data. Competitor to Curl?

## Purpose
qdl's sole purpose is to be a fast and convenient terminal download client.

## Usage

`qdl [URI]` prints out the contents of the URI onto the terminal. This is useful for pipelining.
- Example:
  `$ qdl https://sh.rustup.rs | sh`
Alternately, you can include a second argument for the file name. This downloads the data in a file named as the 2nd argument. This is essentially the same as `qdl [URI] > [FILENAME]`
`qdl [URI] [FILENAME]`.
- Example:
  `$ qdl https://sh.rustup.rs rustsetup.sh`
  Now, accessing the saved file
  `$ nano ./rustsetup.sh`
