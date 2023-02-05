# Async Rust example
> Road to the asynchronous Rust
## Table of Contents

- [About the Project](#about-the-project)
  * [Screenshots](#screenshots)
  * [Tech Stack](#tech-stack)
  * [Features](#features)
- [Getting Started](#getting-started)
  * [Prerequisites](#prerequisites)
  * [Clone & Run](#clone-and-run)
- [Usage](#usage)
  * [Web Server](#web-server)
  * [Notification Daemon](#notification-daemon)
- [Roadmap](#roadmap)
  * [Zero stage](#zero-stage)
  * [First stage](#first-stage)
  * [Second stage](#second-stage)
  * [Third stage](#third-stage)
- [FAQ](#faq)
- [Contact](#contact)

<!-- About the Project -->
## About the Project


<!-- Screenshots -->
### Screenshots

![notif daemon example](https://github.com/khadievedem/rust_async/blob/imgs/notif_daem_blur.jpg?raw=true)

<!-- Tech stack -->
### Tech Stack
 - [Zbus (D-Bus crate)](https://gitlab.freedesktop.org/dbus/zbus)
 - [Console Engine](https://github.com/VincentFoulon80/console_engine)
 - [Tokio-rs](https://github.com/tokio-rs/tokio)
 - [Futures-rs](https://github.com/rust-lang/futures-rs)
 - [Mutex](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
 - [Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html)


<!-- Features -->
### Features
<details>
  <summary>Notification daemon</summary>
  <p>

  - usign D-Bus for catching notifications.
  - using tokio for async.
  - draw simple but stylish boxes around notifications.
  - supports window resizing.
  - supports immortal notifications as well as mortal.
  - supports all apps using D-Bus interface.
    
  </p>
</details>

<details>
  <summary>Web server</summary>
  <p>

  - serving requests concurrently.

  </p>
</details>

<!-- Getting Started -->
## Getting Started

<!-- Prerequisites -->
### Prerequisites
#### Install `rustup`:
> Read ["Installation"] from ['The Rust Programming Language' book].

["Installation"]: https://doc.rust-lang.org/book/ch01-01-installation.html
['The Rust Programming Language' book]: https://doc.rust-lang.org/book/index.html

<!-- Clone and Run -->
### Clone and run

```sh
$ git clone https://github.com/khadievedem/rust_async.git
$ cd rust_async
```
Run notification daemon project with
```sh
$ cargo run --bin notification_daemon
```
or web server
```sh
$ cargo run --bin web_server
```

<!-- Usage -->
## Usage

<!-- Web server -->
#### Web Server

```http
localhost:7878/
```
> hello world page `./web_server/src/hello.html`
```http
localhost:7878/sleep
```
> sleep for a 10 seconds (in order to test concurrency)
```http
localhost:7878/smthfkwejfllkdk
```
> Ooops! :-) `./web_server/src/404.html`

<!-- Notification daemon -->
#### Notification daemon

Run the app according to [Clone & Run](#clone-and-run) section. 
```
press 'q' to exit.
```


<!-- Roadmap -->
## Roadmap

<!-- Zero stage -->
#### Zero stage

* [x] Huge absorption of information about programming in Rust.
* [x] Read Rust async-book.

<!-- First stage -->
#### First stage

* [x] Create web-server following guide in the Rust async-book.
* [x] Migrate from async-std to `tokio` (`web-server`)
* [x] Brainstorm on example project topics.
* [x] Decide which project will best demonstrate async working (under Linux)

> See more about my decision at: [FAQ](#----) section

<!-- Second stage -->
#### Second stage

* [x] Read how communication between apps works in Linux.
* [x] Study the D-Bus Notifications Specification.
* [x] Create simple application using `zbus` (crate for D-Bus)
* [x] Write an interface for catching notifications.
* [x] Brainstorm on showing notifications to the user.
* [x] Unsafe `QT` drawing attempts.
* [x] `GTK` drawing attempts.

> See about problems with GUI at: [FAQ](#----) section

* [x] TUI drawing using `console-engine`.

<!-- Third stage -->
#### Third stage

* [x] Brainstorm on concurrent connection of drawing and catching notifications.
* [x] Async notification catching.
* [x] Async notification boxes drawing.
* [x] Set up communication between catcher and drawer using `tokio mspc` channel.

<!-- FAQ -->
## FAQ

#### Why did you choose the notification daemon?

> My very first idea was to implement my own Windows Manager. But there was one major problem - asynchronous Windows Manager sounds really weird, it must use multithreading. So I started looking for other interesting ideas to implement. \
Notification daemon was selected, because in addition to async programming I learn in detail D-Bus, as well as writing an application that interacts with other programs.

#### Why didn't you implement the GUI? The TUI doesn't look very useful.

> As you can see in [qt-notifications branch](https://github.com/khadievedem/rust_async/tree/qt-notifications) I've tried to implement graphical notificaitons using `qt` as well as `gtk`. But I faced a problem connected with my Windows Manage (I use 'sway' BTW). Tiling WMs can't draw a pop-up window in the corner. Instead they draw it right in the center. To draw notifications in the corner, I would have to work with `winit`, but that would take a very long time. After all, the main goal of the project -> learn async Rust, not to work with graphics. That's why I chose the Terminal application.


<!-- Contact -->
## Contact

#### Edem Khadiev - [Telegram](https://eekhdv.t.me/) - khadiev.edem@gmail.com
