Attention ❗❗❗
---------------

This project has been **ARCHIVED** and we are **NO LONGER** accepting new pull requests or issues. Thank you to all the developers who contributed to this project in the past. If you have any ideas, discussions or submissions regarding the Rust client, **please move over to [rocketmq-clients](https://github.com/apache/rocketmq-clients)**. Thank you!

Introduction
--------------

Project rocketmq-client-rust is targeted to implement Rust binding in native Rust code. At the current moment, it is still a work-in-progress project. Do not use it in production till it grows mature enough.

Architecture
--------------

Basically, this project would follow the same paradigm of [rocketmq-client-cpp v5.0.0](https://github.com/apache/rocketmq-client-cpp/tree/main). Namely, we would build the whole client following protocols described in [rocketmq-apis](https://github.com/apache/rocketmq-apis) on top of [tokio](https://tokio.rs/), utilizing [Protocol buffers](https://developers.google.com/protocol-buffers) to serialize and deserialize data in transmission.

How to build
-----------------

We would use cargo to build, run and test.

License
------------------

This project follows [Apache License Version 2.0](./LICENSE).

How to contribute
------------------

Similar to other Apache RocketMQ projects, we welcome contributions in various ways, from filing a bug report, correcting type error, document writing to complete feature implementation. Any attempt to make this project better is welcome.

If this project catches your attention, do not hesitate to make a pull request.
