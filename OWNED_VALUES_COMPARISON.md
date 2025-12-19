# Owned Values Implementation: Tradeoffs and Comparison

This document explains the owned values implementation, its tradeoffs, and compares it to both the static lifetimes with `Box::leak()` approach and the `Arc`-based implementation.

## Implementation Overview

The owned values approach uses Rust's ownership system directly, with each struct owning its dependencies rather than borrowing or sharing them via `Arc`.

### Key Changes

1. **Struct Definitions**: Removed lifetime parameters, use owned types
2. **Clone Implementation**: All service types implement `Clone` to enable sharing
3. **Main Function**: Services are cloned when needed for multiple consumers

### Code Structure

```rust
// Structs own their dependencies
pub struct CarDataApiImpl {
    pub http_requester: TelemetryHttpRequester,  // Owned
    pub uri: String,                              // Owned
}

pub struct EventSyncImpl {
    pub api: CarDataApiImpl,                      // Owned
    pub redis: RedisImpl,                         // Owned
    pub delay_config: EventSyncConfig,           // Owned
    pub tx: Arc<dyn ChannelQueue>,                // Still Arc (needs sharing)
}

// In main.rs - clone when sharing
let api = CarDataApiImpl { ... };
let event_sync = EventSyncImpl {
    api: api.clone(),  // Clone for EventSyncImpl
    // ...
};
// api is still available here for other uses
```

## Tradeoffs of Owned Values

### Advantages

1. **No Lifetime Parameters**: Cleaner code without `<'a>` annotations
2. **Clear Ownership**: Each struct owns its dependencies, making ownership flow explicit
3. **No Reference Counting Overhead**: No atomic operations for `Arc`
4. **Simpler Mental Model**: Easier to reason about for developers new to Rust
5. **Better for Single-Threaded Code**: No need for `Arc` if not sharing across threads
6. **No Memory Leaks**: Proper cleanup when values are dropped

### Disadvantages

1. **Cloning Overhead**: Services must be cloned when shared:
   - `TelemetryHttpRequester`: Zero-sized, essentially free
   - `String` (URI): ~25 bytes, small allocation
   - `EventSyncConfig`: 56 bytes, stack copy
   - `RedisImpl`: **Expensive** - clones connection pool and state
   - `CarDataApiImpl`: Clones all fields (including `String`)

2. **Ownership Constraints**: Cannot use the same instance in multiple places without cloning:
   ```rust
   let api = CarDataApiImpl { ... };
   let event_sync = EventSyncImpl { api };  // api moved
   api.get_session(...);  // ERROR: api was moved
   ```

3. **Memory Usage**: Each clone creates a new instance:
   - Multiple `RedisImpl` instances = multiple connection pools
   - Multiple `String` copies = multiple heap allocations

4. **May Not Work for All Types**: If a type cannot be cloned (e.g., some connection types), this approach won't work

## Comparison: Three Approaches

### 1. Static Lifetimes with `Box::leak()`

**Implementation:**
```rust
let uri: &'static str = Box::leak(Box::new(String::from("..."))).as_str();
let api: &'static CarDataApiImpl = Box::leak(Box::new(CarDataApiImpl {
    http_requester: &http_requester,
    uri: &uri,
}));
```

**Characteristics:**
- ✅ No lifetime parameters needed
- ✅ Works with async
- ❌ **Memory leak** - data never freed
- ❌ Hides actual ownership model
- ❌ Difficult to test (static references)
- ❌ Not idiomatic Rust

**Performance:**
- No cloning overhead
- No reference counting overhead
- Memory never freed (leak)

### 2. Arc-Based Implementation

**Implementation:**
```rust
let uri = Arc::new(String::from("..."));
let api = Arc::new(CarDataApiImpl {
    http_requester: http_requester.clone(),
    uri: uri.clone(),
});
```

**Characteristics:**
- ✅ No lifetime parameters needed
- ✅ Works with async
- ✅ Automatic memory management
- ✅ Thread-safe sharing
- ✅ Idiomatic Rust
- ⚠️ Atomic reference counting overhead (~20 cycles per clone/drop)
- ⚠️ 16 bytes overhead per `Arc`

**Performance:**
- No data cloning (just reference counting)
- Atomic operations for reference counting
- Memory freed when last `Arc` drops

### 3. Owned Values (Current Implementation)

**Implementation:**
```rust
let uri = String::from("...");
let api = CarDataApiImpl {
    http_requester: http_requester.clone(),
    uri: uri.clone(),
};
```

**Characteristics:**
- ✅ No lifetime parameters needed
- ✅ Clear ownership model
- ✅ No reference counting overhead
- ✅ Works with async (by moving/cloning)
- ⚠️ Cloning overhead for shared services
- ⚠️ Multiple allocations for cloned services

**Performance:**
- Full data cloning when sharing
- No atomic operations
- Memory freed when each instance drops

## Detailed Performance Comparison

### Scenario: Sharing `RedisImpl` between `EventSyncImpl` and `WebsocketImpl`

#### Static Lifetimes (`Box::leak`)
```rust
let redis: &'static RedisImpl = Box::leak(Box::new(RedisImpl::default()));
// Cost: 1 allocation, never freed
// Sharing: Free (just reference)
```

#### Arc Implementation
```rust
let redis = Arc::new(RedisImpl::default());  // 1 allocation
let event_sync_redis = redis.clone();        // Atomic increment (~20 cycles)
let websocket_redis = redis.clone();         // Atomic increment (~20 cycles)
// Cost: 1 allocation + 2 atomic ops + 16 bytes overhead
// Sharing: Very cheap (just reference counting)
```

#### Owned Values
```rust
let redis = RedisImpl::default();            // 1 allocation
let event_sync_redis = redis.clone();        // Full clone (expensive!)
let websocket_redis = redis.clone();         // Another full clone (expensive!)
// Cost: 3 allocations (connection pools duplicated!)
// Sharing: Expensive (full data copy)
```

### Performance Breakdown by Type

| Type | Static Lifetimes | Arc | Owned Values |
|------|------------------|-----|--------------|
| **TelemetryHttpRequester** | 0 bytes, leak | 16 bytes overhead | 0 bytes, free clone |
| **String (URI)** | ~25 bytes, leak | 16 bytes + data | ~25 bytes per clone |
| **EventSyncConfig** | 56 bytes, leak | 16 bytes + data | 56 bytes per clone |
| **RedisImpl** | 1 instance, leak | 1 instance + ref counting | Multiple instances |
| **CarDataApiImpl** | 1 instance, leak | 1 instance + ref counting | Multiple instances |

## When to Use Each Approach

### Use Static Lifetimes (`Box::leak`) When:
- ❌ **Never** - This is an anti-pattern
- Only acceptable for prototyping or when you truly need data for the entire program lifetime

### Use Arc When:
- ✅ Services are expensive to clone (`RedisImpl`, connection pools)
- ✅ Services are shared across many async tasks
- ✅ You need thread-safe sharing
- ✅ You want idiomatic Rust
- ✅ Memory efficiency is important (avoid duplicating large structures)

### Use Owned Values When:
- ✅ Services are cheap to clone (zero-sized, small structs)
- ✅ Services are used sequentially (not shared)
- ✅ Single-threaded code (no need for `Arc`)
- ✅ You want the simplest ownership model
- ✅ Cloning overhead is acceptable

## Real-World Analysis for This Codebase

### Current Dependencies:

1. **TelemetryHttpRequester**: Zero-sized struct
   - **Owned Values**: Free to clone ✅
   - **Arc**: 16 bytes overhead ❌

2. **String (URI)**: ~25 bytes
   - **Owned Values**: Small clone cost ⚠️
   - **Arc**: Small overhead, but better if shared ✅

3. **EventSyncConfig**: 56 bytes (7 u64s)
   - **Owned Values**: Small clone cost ⚠️
   - **Arc**: Small overhead, but better if shared ✅

4. **RedisImpl**: Contains connection pool
   - **Owned Values**: **Very expensive** to clone ❌
   - **Arc**: Cheap sharing ✅

5. **CarDataApiImpl**: Contains `TelemetryHttpRequester` + `String`
   - **Owned Values**: Moderate clone cost ⚠️
   - **Arc**: Better if shared ✅

### Recommendation for This Codebase

**Arc is the better choice** because:

1. **RedisImpl is expensive to clone**: Cloning connection pools is wasteful
2. **Services are shared**: `api` is used in both `main` and `EventSyncImpl`, `redis_client` is used in both `EventSyncImpl` and `WebsocketImpl`
3. **Async context**: Services are moved into async tasks, where `Arc` shines
4. **Memory efficiency**: One `RedisImpl` instance vs multiple cloned instances

**Owned Values would be better if:**
- Services were not shared (each component gets its own instance)
- Services were very cheap to clone
- You wanted the simplest possible ownership model

## Conclusion

The owned values implementation is a valid approach that:
- ✅ Eliminates lifetime parameters
- ✅ Provides clear ownership semantics
- ✅ Avoids memory leaks
- ⚠️ Requires cloning when sharing services
- ⚠️ Can be expensive for large/complex services

**For this specific codebase, `Arc` is the better choice** due to:
1. The expensive nature of cloning `RedisImpl`
2. The need to share services across async tasks
3. Better memory efficiency

However, owned values are a good fit when:
- Services are cheap to clone
- Services are not shared extensively
- You want the simplest ownership model

The static lifetimes approach should be avoided entirely as it's an anti-pattern that causes memory leaks and hides the actual ownership model.

