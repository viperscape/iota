This is a (primarily) UDP protocol spec which includes authentication and integrity by default. It will offer additional features such as Guaranteed at least Once, and Batching to simulate TCP features when needed. Authentication/integrity makes use of the HMAC standard with, by default SHA-256 secure hashing algorithm.

This is currently under development and likely much will change, including some of the spec itself. Feel free to PR at any time, or submit an issue to start a discussion.

Please see the [docs](docs/) (which will soon reflect what the protocol aspires) and the [example binary](src/bin/) which I am using to feel out the library; lastly check out some of the [tests](tests/) to see how things might piece together.
