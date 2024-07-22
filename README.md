# Rusty Development Platform

A rust CLI-based no-frills IDP that is GitOps opinionated and developer centric:

1. What does the developer want?
2. Where does the developer want to deploy? 
3. What capabilities of interest does the developer want to enable? Each target deployment has a defined set of capabilities
2. Version this as code using YAML for a GitOps based process
3. I don't want to force a UI/UX on you. Use the CLI as an API, and manage the UI/UX yourself, because all of you have slightly different process and usage requirements.

## How to use it

### Args

| Arg                | Value                                            |
|:-------------------|:-------------------------------------------------|
| -d or --dry-run    | Sets dry-run mode to true. NO CHANGES ARE PUSHED |
| -c --contract-file | The path to your contract YAML file              |

```shell
cargo run -- -d --contract-file my-idp.yaml
```

### Contract file template

```yaml
action: new

golden-path:
  url: url...
  path: gp/golang/gochi
  branch: main

code:
  github:
    repo: my-app-repo
    branch: main

deploy:
  kubernetes:
    cluster: url
  azure:
    webapp:
      ... azure info
    faas:
      ... azure info
```