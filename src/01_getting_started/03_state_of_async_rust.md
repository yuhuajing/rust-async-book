# The State of Asynchronous Rust

异步 Rust 的某些部分具有与同步 Rust 相同的稳定性保证。其他部分仍在不断发展，将随时间而改变。使用异步 Rust，您可以期望：

- 针对典型并发工作负载的出色运行时性能。
- 更频繁地涉及高级语言功能，例如生命周期（lifetimes）和固定（pinning）。
- 一些兼容性限制，既在同步和异步代码之间，也在不同的异步运行时之间。
- 更高的维护负担，由于异步运行时和语言支持的不断演进。

简而言之，异步 Rust 比同步 Rust 更难使用，可能导致更大的维护负担，但作为回报，它为您提供了最佳性能。异步 Rust 的所有领域都在不断改进，因此这些问题的影响会随着时间而减小。

## Language and library support

While asynchronous programming is supported by Rust itself,
most async applications depend on functionality provided
by community crates.
As such, you need to rely on a mixture of
language features and library support:

- The most fundamental traits, types and functions, such as the
  [`Future`](https://doc.rust-lang.org/std/future/trait.Future.html) trait
  are provided by the standard library.
- The `async/await` syntax is supported directly by the Rust compiler.
- Many utility types, macros and functions are provided by the
  [`futures`](https://docs.rs/futures/) crate. They can be used in any async
  Rust application.
- Execution of async code, IO and task spawning are provided by "async
  runtimes", such as Tokio and async-std. Most async applications, and some
  async crates, depend on a specific runtime. See
  ["The Async Ecosystem"](../08_ecosystem/00_chapter.md) section for more
  details.

Some language features you may be used to from synchronous Rust are not yet
available in async Rust. Notably, Rust does not let you declare async
functions in traits. Instead, you need to use workarounds to achieve the same
result, which can be more verbose.

## Compiling and debugging

大部分情况下，异步 Rust 中的编译器和运行时错误的工作方式与 Rust 中一直如此的方式相同。但有一些值得注意的区别：

### Compilation errors

异步 Rust 中的编译错误遵循与同步 Rust 相同的高标准，但由于异步 Rust 通常依赖于更复杂的语言特性，如生命周期和固定，您可能会更频繁地遇到这些类型的错误。

### Runtime errors

每当编译器遇到异步函数时，它在底层生成一个状态机。异步 Rust 中的堆栈跟踪通常包含来自这些状态机的详细信息，以及来自运行时的函数调用。因此，在异步 Rust 中解释堆栈跟踪可能比同步 Rust 更复杂一些。

### New failure modes

A few novel failure modes are possible in async Rust, for instance
if you call a blocking function from an async context or if you implement
the `Future` trait incorrectly. Such errors can silently pass both the
compiler and sometimes even unit tests. Having a firm understanding
of the underlying concepts, which this book aims to give you, can help you
avoid these pitfalls.

## Compatibility considerations

异步和同步代码并不总是可以自由结合使用。例如，您不能直接从同步函数中调用异步函数。同步和异步代码也倾向于促进不同的设计模式，这可能会使得将代码组合用于不同环境变得困难。

即使异步代码也不能总是自由结合使用。某些 crate 依赖于特定的异步运行时才能正常工作。如果是这样，通常会在 crate 的依赖列表中指定。

These compatibility issues can limit your options, so make sure to
research which async runtime and what crates you may need early.
Once you have settled in with a runtime, you won't have to worry
much about compatibility.

## Performance characteristics

异步 Rust 的性能取决于您使用的异步运行时的实现。尽管支持异步 Rust 应用程序的运行时相对较新，但它们在大多数实际工作负载下表现得非常出色。

尽管如此，异步生态系统中的大多数都假定了一个 多线程 运行时。这使得难以享受单线程异步应用程序的理论性能优势，即更便宜的同步。另一个被忽视的用例是 延迟敏感任务，对于驱动程序、GUI 应用程序等非常重要。这些任务依赖于运行时和/或操作系统的支持，以便适当地进行调度。未来您可以期望有更好的库支持来满足这些用例。
