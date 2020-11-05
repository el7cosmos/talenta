![GitHub Workflow Status](https://img.shields.io/github/workflow/status/el7cosmos/talenta/Rust?style=flat-square)
[![Crates.io](https://img.shields.io/crates/v/talenta?style=flat-square)](https://crates.io/crates/talenta)
![Crates.io](https://img.shields.io/crates/l/talenta?style=flat-square)

---
- [Installation](#installation)
- [Usage](#usage)
    - [Login](#login)
    - [Request attendance](#request-attendance)
        - [Independent checkin/checkout](#independent-checkincheckout)
    - [Live attendance](#live-attendance)
- [License](#license)
- [Contribution](#contribution)
---

## Installation

### Rust Cargo
```shell script
cargo install talenta
```

### macOS Homebrew
```shell script
brew tap el7cosmos/brew
brew install talenta
```

## Usage

### Login
```shell script
# Interactive
talenta login

# Non-interactive
talenta login --email <EMAIL> --password <PASSWORD>
```

### Request attendance
```shell script
# Interactive, use current date
talenta attendance

# Interactive, change attendance date
talenta attendance --date <DATE>

# Non-interactive, use current date
talenta attendance --checkin-time <TIME> --checkout-time <TIME> --description <description>

# Non-interactive, change attendance date
talenta attendance --date <DATE> --checkin-time <TIME> --checkout-time <TIME> --description <description>
```

#### Independent checkin/checkout
```shell script
# Interactive, use current date and time
talenta attendance checkin
talenta attendance checkout

# Interactive, change attendance date and time
talenta attendance checkin --date <DATE> --time <TIME>
talenta attendance checkout --date <DATE> --time <TIME>

# Non-interactive, use current date and time
talenta attendance checkin --description <description>
talenta attendance checkout --description <description>

# Non-interactive, change attendance date and time
talenta attendance checkin --description <description> --date <DATE> --time <TIME>
talenta attendance checkout --description <description> --date <DATE> --time <TIME>
```

- `DATE`: ISO 8601 date format (`YYYY-mm-dd`)
- `TIME`: Time in format `HH:MM`

### Live attendance

> Planned in **0.4.x**

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