# 21. Capstone: A Multithreaded Web Server

This project makes a tiny HTTP server using only the standard library. It is a learning server, not a production-ready public server: real servers need careful HTTP parsing, timeouts, TLS, observability, and resource limits. Its value is seeing how ownership, threads, channels, trait objects, and `Drop` work together.

## The goal

Listen on `127.0.0.1:7878`, answer a few browser requests, and keep a deliberately slow request from blocking every other request.

In Node.js, `http.createServer` hides the TCP socket and event loop. Here `TcpListener` exposes a stream of incoming TCP connections. HTTP is text sent over that stream:

```text
HTTP/1.1 200 OK\r\n
Content-Length: 11\r\n
\r\n
Hello world
```

The blank line separates headers from the response body. `Content-Length` is the byte length of the body, not the number of characters a human sees.

## First: serve one request at a time

```rust
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn handle_connection(mut stream: TcpStream) {
    let request_line = BufReader::new(&stream).lines().next().unwrap().unwrap();
    println!("{request_line}");

    let body = "Hello world";
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{body}",
        body.len()
    );
    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
}
```

`TcpStream` owns an operating-system socket handle. Dropping the stream closes that handle. `BufReader` temporarily borrows the stream and adds a buffer so reading text lines is efficient.

## Why one slow request is a problem

If `/sleep` takes five seconds, a single-threaded loop cannot begin the next connection until that handler returns. Spawning an OS thread for every request avoids that particular wait, but an unbounded flood of requests could consume memory and CPU creating unbounded threads.

A **thread pool** chooses a fixed number of worker threads at startup. The listener turns each connection into a job; a worker takes one job from a queue and executes it.

![Thread-pool flow](./diagrams/thread_pool.svg)

## What a job type means

```rust
type Job = Box<dyn FnOnce() + Send + 'static>;
```

Read it from inside out:

- `FnOnce()` is a no-argument closure that can run once.
- `dyn FnOnce()` allows jobs made from different closure types.
- `Box` puts that unknown-size closure behind a known-size pointer.
- `Send` proves it is safe to transfer the job to another thread.
- `'static` means the job owns what it needs (or borrows only data that truly lasts for the entire program). A queued job must not borrow a local variable that will disappear before a worker runs it.

The `Box` pointer itself is stored in the channel queue. The closure environment it points to is typically stored on the heap. A worker removes that pointer, calls the closure, and then the closure's captured values are dropped.

## Why workers share `Arc<Mutex<Receiver<Job>>>`

`mpsc` means many producers, single consumer. The listener owns a cloneable `Sender<Job>`; each worker needs access to the one `Receiver<Job>`.

```text
listener ── Sender<Job> ──▶ queue ◀── Mutex ◀── Arc clones ── workers
```

- `Arc` gives several threads shared ownership of the receiver wrapper.
- `Mutex` lets only one worker call `recv()` on that receiver at a time.
- The worker releases the lock before running the job. Holding the lock during the job would accidentally make all work serial again.

## Shut down cleanly

The pool should not leave worker threads running when the pool is dropped:

1. Drop every `Sender<Job>` so the channel is closed.
2. A waiting worker receives an error, leaves its loop, and finishes.
3. Join each worker thread in `Drop` before the pool is gone.

This ordering matters. Joining a worker before closing the last sender can wait forever, because the worker is still waiting for a job.

## Build order and checks

1. Bind a listener and print the first request line.
2. Return one fixed `200 OK` response.
3. Serve `hello.html` and `404.html` based on the route.
4. Add a `/sleep` route to show the serial bottleneck.
5. Build a two-worker pool and enqueue connections.
6. Make shutdown join the workers without hanging.

Use the full project brief at [Tiny Web Server](../challenges/04_projects/07_tiny_web_server.md) to turn these steps into your final project.
