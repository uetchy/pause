# pause

[![asciicast](https://asciinema.org/a/JhN3EzSuEMEAwtkB0KPJZi0w8.svg)](https://asciinema.org/a/JhN3EzSuEMEAwtkB0KPJZi0w8)

## Install

```
cargo install pause
```

## Usage

```
function get-repository() {
  hub api search/repositories?q=$1 | jq -r '.items[0].html_url' | pause | xargs ghq get
}
```
