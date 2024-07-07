# Rust Full Websocket Example


## How to modify to add events
### Modify Main
fn handle_event
add your new matched event to MsgIN::NewEvent => {function_to_run(stream).await},

### Modify msg_in.rs
add NewEvent to enum

### Modify msg_out
add data struct to enum

### Add handler file
created handler_process.rs
add function you want to run on matched event


## How it works
The matching works because of how Rust's `serde` library deserializes JSON data into enums with `#[serde(tag = "event", content = "data", rename_all = "snake_case")]` attributes. The attributes tell `serde` how to match the JSON fields to the enum variants. 

Here's a more detailed explanation:

### Enum Definition with Serde

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data", rename_all = "snake_case")]
pub enum MsgIn {
    Echo(MsgInEchoData),
    GoProcess,
}
```

### Serde Attributes Explanation

1. **`#[serde(tag = "event", content = "data")]`**:
   - This tells `serde` that the JSON representation of the enum will have a field named `"event"` that indicates which variant of the enum to use.
   - The `"data"` field will contain the associated data for the variant.

2. **`#[serde(rename_all = "snake_case")]`**:
   - This converts Rust's CamelCase enum variants into snake_case when serializing/deserializing.
   - For example, `GoProcess` becomes `"go_process"` in the JSON.

### JSON Representation

When you send the following JSON from the client:

```json
{
    "event": "go_process",
    "data": {}
}
```

### Deserialization Process

1. **Extract the `event` Field**:
   - `serde` looks at the `"event"` field in the JSON, which is `"go_process"`.

2. **Match the `event` Field**:
   - It matches the value `"go_process"` to the enum variant `GoProcess` because of the `rename_all = "snake_case"` attribute.

3. **Extract the `data` Field**:
   - For `GoProcess`, there is no associated data, so it just confirms the `"data": {}` part of the JSON.

### Enum Variant Matching

Here’s the step-by-step process:

1. **JSON Message**:
   ```json
   {
       "event": "go_process",
       "data": {}
   }
   ```

2. **Deserialization**:
   - `serde` sees `"event": "go_process"` and matches it to `MsgIn::GoProcess`.

3. **Event Handling in `handle_event`**:
   ```rust
   async fn handle_event(stream: &mut WebSocketStream<TcpStream>, event: MsgIn) -> Res<()> {
       match event {
           MsgIn::Echo(data) => {
               stream.send(MsgOut::Echo(handle_echo(&data)).to_msg()).await?
           },
           MsgIn::GoProcess => {
               process_this(stream).await;
           }
       };

       Ok(())
   }
   ```

### Summary

- The `MsgIn` enum uses `serde` attributes to map JSON fields to enum variants.
- The JSON field `"event": "go_process"` maps to the `MsgIn::GoProcess` variant due to the `rename_all = "snake_case"` attribute.
- The `handle_event` function matches the `MsgIn::GoProcess` variant correctly and calls the `process_this` function.

This is how the deserialization and matching process works seamlessly, allowing the `handle_event` function to recognize and handle the `go_process` event correctly.




### This is a boilerplate Rust Websocket Server with:

- socket.io-like message format:
  ```json
  {
    "event": "SOME_EVENT_NAME",
    "data"?: <ARBITRARY_DATA>
  }
  ```
- **JSON serialization** / **deserialization** to / from structs with [serde](https://docs.rs/serde/latest/serde/) and [serde_json](https://docs.rs/serde_json/latest/serde_json/)
  - See [msg_in.rs](src/msg/msg_in.rs) for a list of all incoming events
- **async** / **await** event handlers with [tokio-tungstenite](https://docs.rs/tokio-tungstenite/latest/tokio_tungstenite/)
- **argument parsing** with [clap](https://docs.rs/clap/latest/clap/)
- **logging** with [log](https://docs.rs/log/latest/log/) and [env_logger](https://docs.rs/env_logger/latest/env_logger/)

### To run:

```sh
cargo run
```

### To test:

- Open a new tab in Chrome
- Copy-paste the code from [test-server.js](scripts/test-server.js)

### To modify:

- Add your own incoming and outgoing messages to [msg_in.rs](src/msg/msg_in.rs) and [msg_out.rs](src/msg/msg_out.rs) respectively
- Add your own message handlers to `src/handler`
- Register message handlers in [main.rs](src/main.rs)::handle_event
- Add args in [args.rs](src/args.rs)

### Misc:

- This was based off of [autobahn-server](https://github.com/snapview/tokio-tungstenite/blob/master/examples/autobahn-server.rs).
- This code can be slightly modified to become a CLIENT instead of a SERVER. See [autobahn-client](https://github.com/snapview/tokio-tungstenite/blob/master/examples/autobahn-client.rs).
