# ask

[![asciicast](https://asciinema.org/a/JhN3EzSuEMEAwtkB0KPJZi0w8.svg)](https://asciinema.org/a/JhN3EzSuEMEAwtkB0KPJZi0w8)

```
function get-repository() {
  hub api search/repositories?q=$1 | jq -r '.items[0].html_url' | ask | xargs ghq get
}
```
