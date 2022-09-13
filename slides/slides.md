# How to build a Rust Chat Server

--- 

# About me
- Name: Lucas Meurer
- Working for newcubator since 2020
- Made:
  - Android Apps: 
    - https://play.google.com/store/apps/developer?id=Lucas+Meurer
  - https://malmal.io
    - Made using lots of TypeScript
  - https://hellopaint.io
    - Lots of TypeScript and Rust
  - https://hurlurl.com
    - Pure Rust

---

# Why I learned Rust

- I needed a performant, crash resistant and safe backend for HelloPaint 
- Previous backend in Go was
  - Also fast
  - Did crash from time to time
  - Not fun to work with (I personally dislike Go)

---

# Questions

- Who has heard of Rust before?
- Who has tried Rust before?
  - How'd it go?

---

# End Goal

- We'll build a working chat server with group chats
- We'll learn basics about rust and rust concurrency

---

# Rust Setup

Install rust via https://rustup.rs/

Optional: Install Extension for your favourite IDE
 - https://www.jetbrains.com/de-de/rust/
 - https://code.visualstudio.com/docs/languages/rust
---

# Some Rust basics
While rust is installing, let's talk about Ownership
> Ownership is Rust’s most unique feature
> 
> ~ *rust book*


- Rust has no garbage collector
- To be memory safe, rust has the ownership model
- Every value has an *owner*
- There can only be one owner
- The value is dropped when the owner goes out of scope

---

# Borrowing
- We can borrow a value from an owner to get a reference
- A reference is a bit like a pointer
- There can only be one mutable reference or multiple immutable references

```rust
fn main() {
    let static_string = "Hello";                                // &str, static reference
    let mut hello = static_string.to_string();                  // String, owner
    let ref_1 = &hello;                                         // &String, reference
    let ref_2 = &hello;                                         // &String, reference
    // This will be an error because hello is already borrowed
    let mut_ref = &mut hello;                                   // &mut String, mutable reference
    println!("{hello}, {ref_1}, {ref_2}, {mut_ref}");
    mut_ref.push_str(" World"); 
}
```

--- 

# Traits
Traits are similar to interfaces in other languages
- We won't write our own traits in this workshop but we'll use some built in traits
- Often encountered traits are Clone, Debug, Serialize, Deserialize
- Some traits can be implemented with a derive macro:

```rust
#[derive(Clone, Debug)]             // Adds implementations for Clone and Debug traits
struct Test {
    text: String,                   // This only works if every member also implements Clone and Debug
}

fn main() {
    let test = Test {
        text: "Hello".to_string(),
    };
    println!("{test:?}");           // Will print: Test { text: "Hello" }
    let clone = test.clone();       // Calls clone on all members
}
```

Instead of using derive we could also add a custom implementation for a trait, e.g. to customize println format.

---

# Checkout the github repo
Contains a cargo workspace with the chat ui
https://github.com/lucasmerlin/rust-chat-app
- cargo is rusts package manager, like npm
- cargo workspaces is similar to nx, lerna, turborepo
```bash 
├── Cargo.lock             # Dependency lock file
├── Cargo.toml             # Workspace configuration
└── app                    # Ui package
    ├── Cargo.toml         # Project configuration
    └── src
        ├── chat_ui.rs
        ├── connection.rs
        ├── main.rs
        └── models.rs
```
---

# Start the existing ui

```bash
cargo run -p app                              # Runs the package "app"
```

You should now see the chat ui.

#### -- OR --
<br>

# Start the web app
```bash
cargo install --locked trunk                  # Trunk is a bundler for rust, a bit like webpack or parcel
rustup target add wasm32-unknown-unknown      # Install the wasm target, so we can compile to wasm

cd app                                        # The app folder contains the index.html file needed by trunk
trunk serve                                   # Compiles to wasm and serves on localhost:8080
```

--- 

# Create chat server package

```bash
cargo new server
```

You should see a warning like this:
> current package believes it's in a workspace when it's not:

Add the package to the root Cargo.toml to fix it:
```toml
[workspace]

members = [
    "app",
    "models",
    "server",
]
```

Run it with `cargo run -p server`

--- 

# Add needed dependencies

Edit `server/Cargo.toml`
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }  # Async runtime for rust
tungstenite = "0.17"                            # WebSocket server library
tokio-tungstenite = "0.17"                      
serde = "1"                                     # Serialization
serde_json = "1"                                # Json Serialization
futures-util = "0.3"                            # Async utils

models = { path = "../models"}                  # Contains models 
```

When you run again, it should automatically download and compile dependencies:

```bash
cargo run -p server
```

---

# Open a tcp port

Edit `server/src/main.rs`
```rust
use std::io::Error;
use tokio::net::TcpListener;

// Rust macro that creates a tokio async runtime
#[tokio::main]
async fn main() -> Result<(), Error> {              // Rust uses the Result enum for error handling
    let addr = "0.0.0.0:6789".to_string();

    let listener = TcpListener::bind(addr).await?;  // ? accesses the value in the result or returns an error

    Ok(())                                          // Return Result::Ok with an empty tuple
}
```
 
Rust has two string types:
- str: A fixed size, immutable string, mostly seen as &str
- String: A mutable string type, a bit like Java's StringBuilder
String literals are always &str, basically a reference into the programs binary.
To get a String instance, call `.to_string()`

---

# Accept connections
```rust
use futures_util::{TryStreamExt};                     // We must import the trait when using a trait's function

async fn main() -> Result<(), Error> {
    [...]
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }
    Ok(())
}

async fn accept_connection(stream: TcpStream) -> Result<(), tungstenite::Error> {
    let mut ws_stream = tokio_tungstenite::accept_async(stream).await?;

    if let Some(message) =  ws_stream.try_next().await? {
        println!("{:?}", message);
    }
    Ok(())
}
```

If you now run the server and the client, you should be able to connect and the server should log messages you send.

---

# The Protocol
To underdstand how server and client communicate, let's take a look at `models/src/lib.rs`:
```rust
use serde::{Deserialize, Serialize};                // Serde is a de/serialization crate for rust

#[derive(Serialize, Deserialize, Debug, Clone)]     // Macro that adds implementation for a trait
#[serde(tag = "type")]                              // Json objects will have a e.g. "type": "Connect" field
pub enum ClientMessage {
    SendMessage {                                   
        text: String,
    },
    Connect {                                       // Sent after connecting to the server
        room: String,                               
        user: String,
    },
}
```

---

# Deserialize Messages
Let's parse the received messages with serde_json and get the value with pattern matching:
```rust {1,5-11}
use models::ClientMessage;
[...]

    if let Some(message) =  ws_stream.try_next().await? {
        if let tungstenite::Message::Text(text) = message {                         // Get the ws message text content
            if let Ok(message) = serde_json::from_str::<ClientMessage>(&text) {     // Parse json
                if let ClientMessage::Connect { room, user } = message {            // Check if msg is of type Connect
                    println!("{user} is trying to join {room}");
                }
            }
        }
    }
```
One can also match nested structs: 
```rust
    if let Some(tungstenite::Message::Text(text)) = ws_stream.try_next().await? {
        if let Ok(ClientMessage::Connect { room, user }) = serde_json::from_str::<ClientMessage>(&text) {
            println!("{user} is trying to join {room}");
        }
    }
```

---

# Room and connection modules
We'll create a new module for the room and for the connection related code:
- Create the files `room.rs` and `connection.rs`.
- Add the modules to the top of `main.rs`:
```rust
mod room;
mod connection;

use std::io::Error;
[...]
```

--- 

# Actor pattern
- Actors and Handles
- Actors run in their own task, thread or even on a seperate server
- Handles and actors communicate via message passing

<br/>
<br/>

### -> Our rooms will be actors

--- 

# Room actor
Add structs and enums for the room actor in `room.rs`:
```rust
use std::collections::HashMap;
use tokio::sync::{mpsc, oneshot};
use models::ClientMessage;
use crate::connection::Connection;

#[derive(Debug)]                                    // Needed so we can printlin!("{:?}", message);
enum RoomMessage {                                  // Handle and actor will communicate with this enum
    Join(Connection, oneshot::Sender<u64>),         // Oneshot channels can only send a single message
    ClientMessage(ClientMessage, String),
    Leave(u64),
}
struct RoomActor {
    rx: mpsc::Receiver<RoomMessage>,                // Multiple Producer Single Consumer channel for message passing
                                                    // The actor is the consumer
    next_connection_id: u64,                        // We'll generate a connection id for each user
    users: HashMap<u64, Connection>,
}
#[derive(Clone)]                                    // We should be able to clone the handle
pub struct RoomHandle {
    tx: mpsc::Sender<RoomMessage>,                  // The handle is the producer
}
```

--- 

# Connection struct
Add a connection struct to `connection.rs`:
```rust
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct Connection {
    pub user: String,
    pub sender: mpsc::Sender<tungstenite::Message>,
}
```

--- 

# Room actor impl

In rust, a struct's functions are in an impl block. Add a run function to the actor:
```rust 
[...]

impl RoomActor {
    pub async fn run(&mut self) {
        while let Some(msg) = self.rx.recv().await {
            match msg {
                RoomMessage::Join(connection, send_conn_id) => {
                    println!("User {} joined", connection.user);
                    
                    self.users.insert(self.next_connection_id, connection);
                    send_conn_id.send(self.next_connection_id).unwrap();
                    self.next_connection_id += 1;
                }
                _ => ()             // We'll handle the other cases later
            }
        }
    }
}
```

---

# Room handle impl
Add a new fn to the room handle. It will spawn a new task and run the actor, then return the room handle.
```rust
impl RoomHandle {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(100);      // channel buffer size, 100 worked well during load testing

        let mut actor = RoomActor {             
            rx,
            users: HashMap::new(),
            next_connection_id: 0,
        };

        tokio::spawn(async move {               // Spawn a new tokio task, so the actor can run in parallel
            actor.run().await
        });

        RoomHandle {
            tx,
        }
    }
}
```

---

# Room join function
Add a function to the Room Handle that let's people join rooms.
```rust {1-2,8-19}
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;
[...]

impl RoomHandle {
    [...]

    pub async fn join(&self, user: String, stream: WebSocketStream<TcpStream>) {
        let (conn_id_tx, conn_id_rx) = oneshot::channel();
        let (sender_tx, mut sender_rx) = mpsc::channel(100);

        self.tx.send(RoomMessage::Join(Connection {
            sender: sender_tx,
            user: user.clone(),
        }, conn_id_tx)).await.unwrap();

        let conn_id = conn_id_rx.await.unwrap();
        println!("{user} successfully joined room with user id {conn_id}")
    }
}
```

--- 

# Global room state
To store our rooms, well use an `Arc<Mutex<HashMap<String, RoomHandle>>`.
- Arc is an atomic reference count
- Mutex is a lock
```rust {2-6,10-14}
[...]
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::room::RoomHandle;

type Rooms = Arc<Mutex<HashMap<String, RoomHandle>>>;               // We can define a type

async fn main() -> Result<(), Error> {
    [...]
    let rooms = Arc::new(Mutex::new(HashMap::new()));

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream, rooms.clone()));     // Clone the Arc to get a new reference
    }                                                               // and pass it to the accept_connection function
    [...]
}


```

---

# Global room state II

```rust {5-12}
async fn accept_connection(stream: TcpStream, rooms: Rooms) -> Result<(), tungstenite::Error> {
    [...]
    if let Some(tungstenite::Message::Text(text)) = ws_stream.try_next().await? {
        if let Ok(ClientMessage::Connect { room, user }) = serde_json::from_str::<ClientMessage>(&text) {
            let handle = {
                let mut rooms = rooms.lock().unwrap();              // Lock the room mutex to get the HashMap
                rooms
                    .entry(room)
                    .or_insert_with(|| RoomHandle::new()).clone()   // Clone the room because the reference can't 
            };                                                      // be held across an .await

            handle.join(user, ws_stream).await;
        }
    }
    [...]
}
```

When joining a room you should now see the following logs on the server:
```bash
User lucas2 joined
lucas2 successfully joined room with user id 0
```

---

# Sending Messages
We'll start a new task to send messages recieved through the `sender_rx` channel
```rust
use futures_util::SinkExt;
[...]

impl RoomHandle {
    pub async fn join(&self, user: String, stream: WebSocketStream<TcpStream>) {
        [...]
        let (mut ws_tx, mut ws_rx) = stream.split();                                    
        let actor_tx = self.tx.clone();                         // Clone tx so it can be moved into task

        tokio::spawn(async move {                               // Spawn new task to send messages
            while let Some(msg) = sender_rx.recv().await {
                if let Err(_) = ws_tx.send(msg).await {         // Kick the user if there was an error
                    actor_tx.send(RoomMessage::Leave(conn_id)).await.unwrap();
                }
            }
        });
    }
}
```

--- 

# Sending Messages II
Add a broadcast function that sends messages to all users
```rust {1,4-14}
use models::{ClientMessage, ServerMessage};
[...]
impl RoomActor {
    async fn broadcast(&mut self, msg: ServerMessage) {
        if let Ok(json) = serde_json::to_string(&msg) {             // Convert our message to json
            for connection in self.users.values() {
                connection.sender.send(tungstenite::Message::Text(json.clone())).await.unwrap();
            }
        }
    }
}
```

---

# Sending Messages III
Update the Join handle to send a join message
```rust {6,10-12}
impl RoomActor {
    pub async fn run(&mut self) {
        while let Some(msg) = self.rx.recv().await {
            match msg {
                RoomMessage::Join(connection, send_conn_id) => {
                    let user = connection.user.clone();

                    [...]

                    self.broadcast(ServerMessage::Joined {
                        user,
                    }).await;
                }
            }
        }
    }
    [...]
}
```

Now when joining a room you should see `<user> joined the room` in the app

---

# Sending Chat Messages

If the user sends a SendMessage message we'll broadcast it to all room members

```rust {6-11}
impl RoomActor {
    pub async fn run(&mut self) {
        while let Some(msg) = self.rx.recv().await {
            match msg {
                [...]
                RoomMessage::ClientMessage(ClientMessage::SendMessage { text }, user) => {
                    self.broadcast(ServerMessage::Message {
                        text,
                        user,
                    }).await;
                }
            }
        }
    }
    [...]
}
```

We still need to read messages sent by the client

---

# Sending Chat Messages II

Spawn another task that listens for new messages from the client and sends them to the actor

```rust
impl RoomHandle {
    [...]

    pub async fn join(&self, user: String, stream: WebSocketStream<TcpStream>) {
        [...]
        
        tokio::spawn(async move {
            while let Some(Ok(tungstenite::Message::Text(text))) = ws_rx.next().await {
                if let Ok(value) = serde_json::from_str::<ClientMessage>(&text) {
                    actor_tx.send(RoomMessage::ClientMessage(value, user.clone())).await.unwrap();
                }
            }
            actor_tx.send(RoomMessage::Leave(conn_id)).await.unwrap();
        });
    }
}
```

We can now send and receive chat messages! Yay!

---

# Remove users

```rust
impl RoomActor {
    pub async fn run(&mut self) {
        while let Some(msg) = self.rx.recv().await {
            match msg {
                [...]
                RoomMessage::Leave(conn_id) => {
                    self.remove_user(conn_id);
                }
                _ => ()                                 // This needs to stay because we didn't handle the 
            }                                           // RoomMessage::ClientMessage(ClientMessage::Connect) case
        }
    }
    [...]
    fn remove_user(&mut self, conn_id: u64) {
        let connection = self.users.remove(&conn_id);
        if let Some(connection) = connection {
            self.broadcast(ServerMessage::Left {
                user: connection.user,
            });
        }
    }
}
```

--- 

# We're done!

Things that could be improved upon:
- Remove empty chat rooms
- Authentication, user list, etc
- Move connection logic into another actor

---

# Further Reading

- https://doc.rust-lang.org/book/
  - Especially the chapter about ownership
- https://ryhl.io/blog/actors-with-tokio/
  - Blog post about actors with rust

--- 

# Testing

Tests in Rust are written in the same File as the code it's testing.
A simple test looks like this:

```rust
#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}
```

The module is optional, it can include helper functions, e.g. for test setup logic.

---

# Example Test

```rust
#[tokio::test]                                  // 
async fn room_actor_join() {                    // 
  let (tx, rx) = mpsc::channel(1);
  let mut actor = RoomActor {
    rx,
    next_connection_id: 0,
    users: HashMap::new(),
  };

  tokio::spawn(async move {
    actor.run().await
  });

  let (ws_tx, mut ws_rx) = mpsc::channel(1);
  let (join_tx, join_rx) = oneshot::channel();
  tx.send(RoomMessage::Join(Connection {
    user: "test".to_string(),
    sender: ws_tx,
  }, join_tx)).await.unwrap();

  let conn_id = join_rx.await.unwrap();
  assert_eq!(conn_id, 0);
  assert_eq!(ws_rx.recv().await.unwrap(), tungstenite::Message::Text(r#"{"type":"Joined","user":"test"}"#.to_string()))
}
```