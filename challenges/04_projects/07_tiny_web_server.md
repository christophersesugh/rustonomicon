# Project 7: Tiny Web Server

**Time:** 4–8 hours
**Practice:** TCP streams, file I/O, closures, trait objects, thread pools, and `Drop`

Build the capstone from [Chapter 21](../../notes/21_multithreaded_web_server.md). Bind only to `127.0.0.1` while learning. This server is for local education, not exposure to the public internet.

## Required behavior

1. Listen on `127.0.0.1:7878`.
2. Return a valid `200 OK` response for `/`.
3. Return a `404 NOT FOUND` response for an unknown route.
4. Add `/sleep`, which waits briefly before replying.
5. Use a fixed-size thread pool so `/` can answer while `/sleep` is running.
6. Drop the pool without hanging: all worker threads must be joined.

## Build order

1. Bind a `TcpListener` and print the first request line.
2. Write one correctly formatted HTTP response.
3. Serve a tiny `hello.html` file and a `404.html` file.
4. Add a `/sleep` route and observe the serial bottleneck with two browser tabs.
5. Define `Job = Box<dyn FnOnce() + Send + 'static>` and enqueue connections.
6. Implement workers and graceful shutdown.

## Acceptance checks

- `curl http://127.0.0.1:7878/` receives an HTTP response.
- An unknown route returns status 404.
- Start `/sleep`, then request `/`; the second request finishes before the sleep route does.
- Dropping the `ThreadPool` returns without a leaked running worker or deadlock.
- The mutex protects only receiving a job, not executing the whole job.

## Stretch goals

- Parse request methods carefully and return 405 for unsupported methods.
- Add a maximum request size and a read timeout.
- Use a queue capacity with backpressure.

## Memory check

Why is a queued job boxed? Why must every `Sender` be dropped before `ThreadPool::drop` joins workers? Explain both without using the word “because Rust says so.”
