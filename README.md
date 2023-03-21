# Copy objects from RocksDB to MongoDB demo

This program will transfer all objects from one RocksDB instance to MongoDB.

## Try it out!
1. Install [Rust](https://rustup.rs/)
2. Set up MongoDB docker instance
```bash
$ docker run --name sui -p 27017:27017/tcp -d mongo:latest
```
3. Run the app
```bash
$ cargo run --release
```
It will create a temporary RocksDB instance, fill it with sample data, and copy those objects to the MongoDB instance.