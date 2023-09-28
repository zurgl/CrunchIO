# Startup scripts

## StartupScriptId

### Type

```rust
type StartupScriptId = Uuid // Or String
```

### Description

>startup script id. you need to first add a startup script to datacrunch via the startup script endpoint

### Example

```json
"26d43424-229e-4b45-bae2-86eb1e919514"
```

## StartupScriptIds

### Type

```rust
type StartupScriptIds = Vec<Uuid>
```

### Description

* Description: array of startup script ids

Example:

```json
[
  "26d43424-229e-4b45-bae2-86eb1e919514",
  "767066ac-78fb-4b06-bbbf-0bbb37b516c7"
]
```
