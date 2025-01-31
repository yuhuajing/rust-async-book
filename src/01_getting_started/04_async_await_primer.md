# `async`/`.await` Primer

`async`/`.await` is Rust's built-in tool for writing asynchronous functions
that look like synchronous code. `async` transforms a block of code into a
state machine that implements a trait called `Future`. 与在同步方法中调用阻塞函数会阻塞整个线程不同，被阻塞的 Future 会让出线程的控制权，允许其他 Future 运行。这使得异步代码可以更高效地处理并发任务，而不会阻塞整个线程。

Let's add some dependencies to the `Cargo.toml` file:

```toml
{{#include ../../examples/01_04_async_await_primer/Cargo.toml:9:10}}
```

To create an asynchronous function, you can use the `async fn` syntax:

```rust,edition2018
async fn do_something() { /* ... */ }
```

The value returned by `async fn` is a `Future`. For anything to happen,
the `Future` needs to be run on an executor.

异步函数返回的是一个 future 的 block，要调用封装的future 返回数据的话，需要执行该block。

```rust,edition2018
{{#include ../../examples/01_04_async_await_primer/src/lib.rs:hello_world}}
```

或者通过 tokio 启动多线程 `tokio = { version = "1", features = ["macros", "rt-multi-thread"] }`
```rust

async fn hello_world() {
    println!("hello, world!");
}

#[tokio::main]
async fn main() {
    let _future = hello_world().await; // Nothing is printed
}
```

Inside an `async fn`, you can use `.await` to wait for the completion of
another type that implements the `Future` trait, such as the output of
another `async fn`. Unlike `block_on`, `.await` doesn't block the current
thread, but instead asynchronously waits for the future to complete, allowing
other tasks to run if the future is currently unable to make progress.

对于多个并发的的任务，通过futures 管理并发的任务

For example, imagine that we have three `async fn`: `learn_song`, `sing_song`,
and `dance`:

```rust,ignore
async fn learn_song() -> Song { /* ... */ }
async fn sing_song(song: Song) { /* ... */ }
async fn dance() { /* ... */ }
```

One way to do learn, sing, and dance would be to block on each of these
individually:

```rust,ignore
{{#include ../../examples/01_04_async_await_primer/src/lib.rs:block_on_each}}
```

However, we're not giving the best performance possible this way—we're
only ever doing one thing at once! Clearly we have to learn the song before
we can sing it, but it's possible to dance at the same time as learning and
singing the song. To do this, we can create two separate `async fn` which
can be run concurrently:

```rust,ignore
{{#include ../../examples/01_04_async_await_primer/src/lib.rs:block_on_main}}
```

In this example, learning the song must happen before singing the song, but
both learning and singing can happen at the same time as dancing. If we used
`block_on(learn_song())` rather than `learn_song().await` in `learn_and_sing`,
the thread wouldn't be able to do anything else while `learn_song` was running.
This would make it impossible to dance at the same time. By `.await`-ing
the `learn_song` future, we allow other tasks to take over the current thread
if `learn_song` is blocked. This makes it possible to run multiple futures
to completion concurrently on the same thread.
