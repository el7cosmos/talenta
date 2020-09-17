![GitHub Workflow Status](https://img.shields.io/github/workflow/status/el7cosmos/talenta/Rust?style=flat-square)
[![Crates.io](https://img.shields.io/crates/v/talenta?style=flat-square)](https://crates.io/crates/talenta)
![Crates.io](https://img.shields.io/crates/l/talenta?style=flat-square)

---
- [Installation](#installation)
- [Usage](#usage)
    - [Login](#login)
    - [Request attendance](#request-attendance)
---

## Installation

### Rust Cargo
```sh
cargo install talenta
```

### macOS Homebrew
```sh
brew install el7cosmos/brew/talenta
```

## Usage

### Login
```sh
# Interactive
talenta login

# Non-interactive
talenta login --email <EMAIL> --password <PASSWORD>
```

### Request attendance
```sh
# Interactive, use current date
talenta attendance

# Interactive, change attendance date
talenta attendance --date <DATE>

# Non-interactive, use current date
talenta attendance --checkin-time <TIME> --checkout-time <TIME> --reason <reason>

# Non-interactive, change attendance date
talenta attendance --date <DATE> --checkin-time <TIME> --checkout-time <TIME> --reason <reason>
```

- `DATE`: ISO 8601 date format (`YYYY-mm-dd`)
- `TIME`: Time in format `HH:MM`

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.