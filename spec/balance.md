# Balance

## Currency

### Description

>Currency type

### Type definition

```rust
enum {
  #[default]
  USD
}
```

## Json formatted Example

```json
usd
```

## UserBalance

### Description

>The user's balance

### Type definition

```rust
struct {
  amount: f64,
  currency: String
}
```

## Json formatted Example

```json
{
  "amount": 50,
  "currency": "usd"
}
```
