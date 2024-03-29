# Rucord - Rust Library for Discord API Interactions

**Note: This library is currently under development and is not yet recommended for production use.**

## Overview

Rucord is a Rust library designed to provide a high-performance and flexible interface for building Discord bots and applications. Built using Rust's async/await syntax, Rucord leverages non-blocking I/O operations to deliver a fast and efficient API for interacting with the Discord API.

Rucord provides a low-level API for granular control over the Discord API, as well as higher-level abstractions for common use cases. It is designed to be easy to use for developers of all levels, while still providing powerful features and scalability for advanced use cases.

## Contributing

We welcome contributions to Rucord! If you're interested in contributing, please read our [contributing guidelines](CONTRIBUTING.md) for more information on how to get started.

We also have a [code of conduct](CODE_OF_CONDUCT.md) that we expect all contributors to adhere to.

## Roadmap

### WebSocket Manager

- [x] Implement worker and shard creation.
- [x] Implement shard bucket creation.
- [x] Resolve hello event and implement heartbeat interval.
- [x] Implement identify message sending.
- [x] Apply identify rate limit = 5 second.
- [x] Resolve all gateway operations.
- [ ] Resolve ws close event.

### Client Structure

- [ ] Implement Client Structure.

## Blazingly Fast 🚀

In the spirit of Rust, Rucord is being built with speed and efficiency in mind. Our goal is to provide a library that is both powerful and blazingly fast. So, get ready to launch your Discord bots into orbit with Rucord! 🚀
