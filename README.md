# ask

```
function get-repository() {
  hub api search/repositories?q=$1 | jq -r '.items[0].html_url' | ask | xargs ghq get
}
```
