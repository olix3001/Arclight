# Arclight's config file

Arclight's config file (`arclight.yml`) contains information about the project, like profiles, libraries to use and project configuration

### Syntax

```yaml
project:
    name: "My project"
    version: "1.0.0"

dependencies:
    Hello: "*"
    World: "1.0"

profiles:
    debug:
        opt-level: 0
        lto: false
        error-handler: true

    release:
        opt-level: 3
        lto: true
```
