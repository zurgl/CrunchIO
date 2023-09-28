# Locations

## Location

### Description

>An object describing a location

### Type definition

```rust
struct {
  code: String,
  name: String,
  country_code: String
}
```

## Json formatted Example

```json
{
  "code": "FIN-01",
  "name": "Finland 1",
  "country_code": "FI"
}
```

## Location code

### Description

>Datacenter location

### Type definition

```rust
enum LocationCode {
  #[default]
  FIN01,
  ICE01,
} 
```

## Json formatted Example

```json
"FIN-01"
```

## Location Availabilities

### Description

>Instance available Datacenter location

### Type definition

```rust
struct {
  location_code: LocationCode,
  availabilites: Vec<InstanceType>
}
```

## Json formatted Example

```json
{
  "location_code": "FIN-01",
  "availabilites": [
    "8V100.48M"
  ]
}
```
