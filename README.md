# i-love-rust-examples
Mostly a convoluted bunch of threads and skeleton code, but hopefully it helps people understand tokio, rayon, websocket, database pooling and async design

## Plans
### Thread1 long running
  
probably reqwest + governor, REST fetch data from polygon on a schedule 3 seconds polling across an HTTPS SSE connection

This is a LONG running thread. The connection MUST be keep-alive to avoid handshake overhead each time.

https://docs.rs/reqwest/0.10.8/reqwest/header/constant.CONNECTION.html

https://booyaa.wtf/wifilocation/reqwest/index.html

Per the docs we need to create a client

Thoughts on middleware.. esp error handling and logging

https://truelayer.com/blog/adding-middleware-support-to-rust-reqwest

### Thread2 long running

I'm told the best container for data may be ndarrays. Parse json and be base container This will prepare the data.

### Thread3 short
Small parallel computation doing basic math on the ndarrays. After computation copy to thread4

### Thread4-6 long running

More advanced analysis and computational math on the list

### Thread7 long running

WebSocket server to copy final data, format in json and push every 3 seconds

### Thread8-12 long running

database connection pool

### Thread13 long running

spider conquest. again reqwest + govenor or scheduling somehow. I don't want to mix this with thread1