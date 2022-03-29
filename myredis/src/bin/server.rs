use bytes::Bytes;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use mini_redis::Command::{self, Get, Set};


type DB = Arc<Mutex<HashMap<String, Bytes>>>;

type ShardedDB = Arc<Vec<Mutex<HashMap<String, Vec<u8>>>>>;

fn new_sharded_db(num_shards: usize) -> ShardedDB {
    let mut db = Vec::with_capacity(num_shards);
    for _ in 0..num_shards {
        db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(db)
}

#[tokio::main]
async fn main() {
    // Bind the Listener
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Mini Redis listening on PORT: 6379...");

    // Create a shared state
    let _db: DB = Arc::new(Mutex::new(HashMap::new()));

    // Create a sharded db
    let shard_db = new_sharded_db(64);

    loop {
        // Listen to the socket
        let (socket, _) = listener.accept().await.unwrap();

        // Clone the handle to the db
        let db = shard_db.clone();

        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: ShardedDB) {
    println!("Mini Redis processing request...");

    // Read Redis frames
    let mut connection: Connection = Connection::new(socket);

    // Read frames
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut hasher = DefaultHasher::new();
                hasher.write(cmd.key().as_bytes());
                let expected: u64 = hasher.finish() % db.len() as u64;
                let mut shard = db[expected as usize].lock().unwrap();
                shard.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".into())
            }
            Get(cmd) => {
                let mut hasher = DefaultHasher::new();
                hasher.write(cmd.key().as_bytes());
                let expected: u64 = hasher.finish() % db.len() as u64;
                let shard = db[expected as usize].lock().unwrap();
                if let Some(v) = shard.get(cmd.key()) {
                    Frame::Bulk(Bytes::from(v.to_vec()))
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("Unsupported command: {:?}", cmd),
        };

        // Write response to client
        connection.write_frame(&response).await.unwrap();
    }

}