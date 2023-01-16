# swath.cc link shortener

I built this link shortener because I wanted a way we could minify really long Google Doc links for the saturday enrichment program I'm part of, where I teach children literary skills.

Built it in Rust because I would like to learn it better! This is my first project in Rust :)

Frontend is in NextJS, backend uses Sled, Salvo and Tokio.

Running the code:
```
git clone https://github.com/edwardwc/link-shortener
cd link-shortener
cargo run --release # for release builds
./target/release/link-shortener
```
