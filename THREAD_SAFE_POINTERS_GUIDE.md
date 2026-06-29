# Rust Thread-Safe Smart Pointers: Learning Guide

## Key Insight: Single-Threaded vs Multi-Threaded

**The Problem:**

```rust
// Single-threaded (won't work across threads):
let data = Rc::new(RefCell::new(vec![1, 2, 3]));

// Try to send to another thread:
thread::spawn(move || {
    println!("{:?}", data.borrow());  // ❌ Error: Rc<T> is not Send/Sync
});
// Rc uses non-atomic ref counting (not thread-safe!)
```

**Why Rc fails:**
- `Rc` uses basic integers for ref counting → not atomic
- Two threads increment simultaneously → race condition!
- `RefCell` uses simple runtime borrow tracking → no locks
- Thread 1 locks, Thread 2 blocks forever

**Thread-safe versions:**

```rust
// Multi-threaded (works across threads):
let data = Arc::new(Mutex::new(vec![1, 2, 3]));

thread::spawn({
    let data = Arc::clone(&data);
    move || {
        let mut guard = data.lock().unwrap();
        guard.push(4);
    }
});
```

**The core difference:**

| Aspect | `Rc<RefCell<T>>` | `Arc<Mutex<T>>` |
|--------|---|---|
| **Ref counting** | Simple integers | Atomic (thread-safe) |
| **Interior mutability** | Runtime borrow check | OS mutex lock |
| **Thread-safe** | ✗ Not Send/Sync | ✓ Send + Sync |
| **Performance** | Very fast (single-thread) | Slower (lock overhead) |
| **When to use** | Single-threaded only | Multi-threaded required |
| **Panic on error** | Borrow panic | Lock poison |

---

## 1. `Arc<T>` - Atomic Reference Counting (Thread-Safe Rc)

**Implementation Difficulty: ⭐⭐ Easy (just replace Rc with Arc)**

**Problem:** Multiple threads need shared ownership of the same data (read-only).

**Essential Methods:**
- `Arc::new(value)` - Create atomic ref-counted pointer
- `Arc::clone(&arc)` - Increment atomic ref count (cheap, thread-safe)
- `Arc::strong_count(&arc)` - Check atomic counter
- Uses atomic operations internally (no manual synchronization)

```rust
// Scenario: Multiple threads read from shared configuration

use std::sync::Arc;
use std::thread;

struct Config {
    db_url: String,
    timeout: u64,
}

let config = Arc::new(Config {
    db_url: "postgresql://localhost".to_string(),
    timeout: 30,
});

let mut handles = vec![];

for i in 0..3 {
    let config = Arc::clone(&config);  // Each thread gets a clone
    let handle = thread::spawn(move || {
        println!("Thread {}: Connecting to {}", i, config.db_url);
        println!("Timeout: {}s", config.timeout);
        // Arc count automatically decremented when thread ends
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}
```

**Key:** No interior mutability! Can't call `.lock()` or mutate through `Arc`.

```rust
// Won't compile:
arc.value = 42;  // ❌ Arc<T> doesn't give &mut
arc.data.push(item);  // ❌ Can't mutate immutably

// Solution: Wrap in Mutex or RwLock
let arc = Arc::new(Mutex::new(value));
arc.lock().unwrap().data.push(item);  // ✓ Works!
```

**Atomic operations (under the hood):**
```rust
// Arc internally uses atomic compare-and-swap:
// When you Arc::clone(&x), it does:
//   atomic_fetch_add(&x.refcount, 1)
// This is thread-safe even with multiple CPUs!
```

**Real-world:** Shared config, cached data, thread-pool work queues.

---

## 2. `Mutex<T>` - Mutual Exclusion Lock (Thread-Safe RefCell)

**Implementation Difficulty: ⭐⭐ Easy (but understand locking!)**

**Problem:** Thread needs exclusive access to mutable data.

**Essential Methods:**
- `Mutex::new(value)` - Create mutex
- `.lock()` - Get exclusive lock (blocks if locked, returns `Result`)
- `.lock().unwrap()` - Get lock or panic
- `.try_lock()` - Non-blocking attempt (returns `Result`)
- `.into_inner()` - Consume mutex, get value

```rust
// Scenario: Multiple threads incrementing a shared counter

use std::sync::{Arc, Mutex};
use std::thread;

struct Counter {
    count: i32,
}

let counter = Arc::new(Mutex::new(Counter { count: 0 }));
let mut handles = vec![];

for _ in 0..5 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        for _ in 0..1000 {
            let mut guard = counter.lock().unwrap();  // ← Exclusive access
            guard.count += 1;
            // guard is automatically released at end of scope
        }
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("Final count: {}", counter.lock().unwrap().count);  // 5000
```

**Lock guard and RAII:**

```rust
let counter = Arc::new(Mutex::new(5));

{
    let mut guard = counter.lock().unwrap();  // ← Lock acquired
    *guard += 1;
    println!("{}", *guard);  // 6
}   // ← Lock released automatically here (RAII)

// Other threads can now lock it
let mut guard2 = counter.lock().unwrap();  // ✓ No deadlock
```

**Deadlock danger:**

```rust
// ❌ Deadlock example (don't do this):
let m = Arc::new(Mutex::new(0));
let m1 = Arc::clone(&m);

// Thread 1 locks m
let lock1 = m.lock().unwrap();

// Thread 2 tries to lock m... FOREVER
thread::spawn(move || {
    let lock2 = m1.lock().unwrap();  // Waits for Thread 1 forever!
});

// To fix: release lock early
{
    let lock1 = m.lock().unwrap();
    // Do work
}  // Release before thread spawns

thread::spawn(move || {
    let lock2 = m1.lock().unwrap();  // ✓ Now works
});
```

**Poison flag:**

```rust
let counter = Arc::new(Mutex::new(0));

thread::spawn({
    let counter = Arc::clone(&counter);
    move || {
        let mut guard = counter.lock().unwrap();
        *guard += 1;
        panic!("Oops!");  // Thread panics while holding lock
        // Lock is marked "poisoned"
    }
}).join().ok();

// Trying to lock poisoned mutex:
match counter.lock() {
    Ok(guard) => println!("Lock acquired"),
    Err(poisoned) => {
        // Lock was held during panic
        println!("Mutex is poisoned!");
        let guard = poisoned.into_inner();  // Can recover the data
        println!("Recovered: {}", guard);
    }
}
```

**When to use:** Shared mutable state with infrequent writes, counters, flags.

---

## 3. `RwLock<T>` - Read-Write Lock (Better for Readers)

**Implementation Difficulty: ⭐⭐⭐ Medium (more complex than Mutex)**

**Problem:** Many threads read, few threads write. Mutex serializes everything.

**Essential Methods:**
- `RwLock::new(value)` - Create read-write lock
- `.read()` - Get shared read lock (many readers, blocks if writing)
- `.write()` - Get exclusive write lock (blocks if reading/writing)
- `.try_read()` / `.try_write()` - Non-blocking versions

```rust
// Scenario: Many threads reading, occasional updates

use std::sync::{Arc, RwLock};
use std::thread;

struct Cache {
    data: Vec<String>,
}

let cache = Arc::new(RwLock::new(Cache { data: vec![] }));

// Spawn 10 reader threads
for i in 0..10 {
    let cache = Arc::clone(&cache);
    thread::spawn(move || {
        let read_guard = cache.read().unwrap();  // Shared read lock
        println!("Thread {}: {:?}", i, read_guard.data);
        // Multiple threads can hold read locks simultaneously!
    });
}

// Spawn 1 writer thread
let cache = Arc::clone(&cache);
thread::spawn(move || {
    thread::sleep(std::time::Duration::from_millis(10));
    let mut write_guard = cache.write().unwrap();  // Exclusive write lock
    write_guard.data.push("new value".to_string());
    println!("Updated cache");
    // All readers block until write completes
});
```

**Read vs Write lock behavior:**

```
RwLock: ┌─────────────────┐
        │ State: Idle     │
        └─────────────────┘
             ↓
   ┌─────────┴─────────┐
   ↓                   ↓
[Read Lock]      [Write Lock]
Can overlap!      Exclusive!

Multiple threads:
- Read lock: ✓ Thread A, ✓ Thread B, ✓ Thread C
- Write lock: ✓ Thread D (all others blocked)
```

**Upgrade/Downgrade (not standard, need library):**

```rust
// Detect-then-act (RwLock issue):
let cache = Arc::new(RwLock::new(vec![]));

let read_guard = cache.read().unwrap();
if read_guard.is_empty() {
    drop(read_guard);  // Release read lock
    // Another thread could have written here!
    let mut write_guard = cache.write().unwrap();  // Check again!
    if write_guard.is_empty() {
        write_guard.push("value".to_string());
    }
}
```

**When to use:** Caches, configuration, analytics—read-heavy workloads.

---

## 4. `Arc<Mutex<T>>` - Shared Mutable State (Thread-Safe Pattern)

**Implementation Difficulty: ⭐⭐⭐ Medium (very common in multi-threaded code)**

**This is the thread-safe equivalent of `Rc<RefCell<T>>`! 🎯**

**Essential Methods:**
- Create: `Arc::new(Mutex::new(value))`
- Share: `Arc::clone(&arc_mutex)`
- Lock & read: `arc_mutex.lock().unwrap().field`
- Lock & write: `arc_mutex.lock().unwrap().field = value`

**Problem:** Multiple threads need to mutate the same data.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

struct GameState {
    players: Vec<String>,
    score: i32,
}

let game_state = Arc::new(Mutex::new(GameState {
    players: vec![],
    score: 0,
}));

let mut handles = vec![];

// Player 1 thread
{
    let state = Arc::clone(&game_state);
    let handle = thread::spawn(move || {
        let mut guard = state.lock().unwrap();
        guard.players.push("Alice".to_string());
        guard.score += 10;
    });
    handles.push(handle);
}

// Player 2 thread
{
    let state = Arc::clone(&game_state);
    let handle = thread::spawn(move || {
        thread::sleep(std::time::Duration::from_millis(5));
        let mut guard = state.lock().unwrap();
        guard.players.push("Bob".to_string());
        guard.score += 15;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

let final_state = game_state.lock().unwrap();
println!("Players: {:?}", final_state.players);  // ["Alice", "Bob"]
println!("Score: {}", final_state.score);        // 25
```

**Comparison with single-threaded:**

```rust
// Single-threaded (Leptos):
let state = Rc::new(RefCell::new(data));
state.borrow_mut().value = 42;

// Multi-threaded equivalent:
let state = Arc::new(Mutex::new(data));
state.lock().unwrap().value = 42;
```

---

## 5. `Arc<RwLock<T>>` - Shared Mutable with Read-Write Locks

**Implementation Difficulty: ⭐⭐⭐ Medium (same as Arc<Mutex>, but more complex)**

**Problem:** Multiple threads read/write, but reads are more common.

```rust
use std::sync::{Arc, RwLock};
use std::thread;

struct Analytics {
    page_views: u64,
    unique_users: u64,
}

let analytics = Arc::new(RwLock::new(Analytics {
    page_views: 0,
    unique_users: 0,
}));

// Many reader threads
for i in 0..5 {
    let analytics = Arc::clone(&analytics);
    thread::spawn(move || {
        let read_guard = analytics.read().unwrap();
        println!("Reporter {}: Views = {}", i, read_guard.page_views);
        // Multiple reporters can read simultaneously!
    });
}

// Few writer threads
let analytics = Arc::clone(&analytics);
thread::spawn(move || {
    thread::sleep(std::time::Duration::from_millis(10));
    let mut write_guard = analytics.write().unwrap();
    write_guard.page_views += 100;
    // All readers are blocked until write completes
    println!("Updated page views");
});
```

**When to use:** Analytics, telemetry, metrics—read-heavy shared mutable data.

---

## 6. `Atomic*` Types - Lock-Free Access to Copy Types

**Implementation Difficulty: ⭐⭐⭐⭐ Hard (requires understanding atomic operations)**

**Problem:** Simple counter/flag accessed by many threads. Mutex overkill.

**Types available:**
- `AtomicBool` - lock-free boolean
- `AtomicU32`, `AtomicU64` - lock-free integers
- `AtomicI32`, `AtomicI64` - signed integers
- `AtomicUsize` - machine-sized unsigned integer

**Essential Methods:**
- `load(Ordering)` - Read value
- `store(value, Ordering)` - Write value
- `fetch_add(delta, Ordering)` - Atomic increment
- `compare_exchange(old, new, success_order, failure_order)` - Compare and swap

```rust
// Scenario: Global request counter (many threads, very frequent updates)

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;

let request_count = Arc::new(AtomicU64::new(0));

let mut handles = vec![];
for i in 0..10 {
    let count = Arc::clone(&request_count);
    let handle = thread::spawn(move || {
        for _ in 0..1000 {
            count.fetch_add(1, Ordering::Relaxed);  // Atomic increment
        }
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("Total requests: {}", request_count.load(Ordering::SeqCst));  // 10000
```

**Ordering types (memory semantics):**

```rust
// Relaxed: Fastest, no synchronization guarantees
atomic.fetch_add(1, Ordering::Relaxed);  // ← Use when you just need atomicity

// Release/Acquire: Medium speed, good for flags
// Release: Write before this becomes visible to other threads
// Acquire: See all writes before the Release
atomic.store(true, Ordering::Release);
if atomic.load(Ordering::Acquire) { ... }

// SeqCst: Slowest, full sequential consistency
atomic.store(42, Ordering::SeqCst);  // ← Safest, use if unsure
```

**When to use:** Counters, flags, reference counting (rarely needed—use Arc!).

---

## 7. `Channels` - Message Passing Between Threads

**Implementation Difficulty: ⭐⭐ Easy (very idiomatic in Rust)**

**Problem:** Send data from one thread to another without sharing state.

**Essential Methods:**
- `channel()` or `mpsc::channel()` - Create sender/receiver
- `.send(value)` - Send message (blocks if buffer full)
- `.recv()` - Receive message (blocks if empty)
- `.try_recv()` - Non-blocking receive
- `crossbeam::channel` - Better ergonomics

```rust
// Scenario: Producer-consumer pattern

use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();

// Producer thread
thread::spawn(move || {
    for i in 0..5 {
        tx.send(i).unwrap();  // Send value
        println!("Sent: {}", i);
    }
});

// Consumer thread (main thread)
for received in rx {
    println!("Received: {}", received);
}
```

**Multiple producers (mpsc = multi-producer, single-consumer):**

```rust
let (tx, rx) = mpsc::channel();

// Clone sender for multiple producers
for i in 0..3 {
    let tx = tx.clone();
    thread::spawn(move || {
        tx.send(format!("From thread {}", i)).unwrap();
    });
}

drop(tx);  // Drop original to signal no more senders

for msg in rx {
    println!("{}", msg);
}
```

**Crossbeam for better ergonomics:**

```rust
use crossbeam::channel;

let (tx, rx) = channel::unbounded();  // No buffer limit

// Both send() and recv() never panic
let result = tx.send(42);
let result = rx.recv();
```

**When to use:** Producer-consumer, worker pools, decoupling threads.

---

## 8. Condition Variables - Thread Coordination

**Implementation Difficulty: ⭐⭐⭐ Medium (complex but powerful)**

**Problem:** Thread needs to wait for a condition set by another thread.

**Essential Methods:**
- `Condvar::new()` - Create condition variable
- `.wait(guard)` - Release lock and wait (reacquires when notified)
- `.notify_one()` - Wake one waiting thread
- `.notify_all()` - Wake all waiting threads

```rust
use std::sync::{Arc, Mutex, Condvar};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let cond = Arc::new(Condvar::new());

// Notifier thread
{
    let counter = Arc::clone(&counter);
    let cond = Arc::clone(&cond);
    thread::spawn(move || {
        for i in 1..=3 {
            thread::sleep(std::time::Duration::from_secs(1));
            let mut guard = counter.lock().unwrap();
            *guard = i;
            cond.notify_one();  // Wake one waiter
            println!("Notified: {}", i);
        }
    });
}

// Waiter thread
let counter = Arc::clone(&counter);
let cond = Arc::clone(&cond);
thread::spawn(move || {
    let mut guard = counter.lock().unwrap();
    while *guard < 3 {
        guard = cond.wait(guard).unwrap();  // Wait and reacquire lock
        println!("Woke up, counter: {}", *guard);
    }
}).join().unwrap();
```

**When to use:** Producer-consumer with pause/resume, synchronization barriers.

---

## Quick Decision Tree

**What problem are you solving?**

```
Multiple threads accessing same data?
  → Yes: Need Arc (atomic ref counting)
      ⭐⭐ Always use Arc, not Rc
  → No: Use local ownership

Need to mutate shared data?
  → Yes:
      → Only integers/bools?
         → Use Atomic*
         ⭐⭐⭐⭐ Advanced, rarely needed
      → Complex types?
         → Mostly reads, few writes?
            → Use Arc<RwLock<T>>
            ⭐⭐⭐ Good for caches
         → Equal reads/writes?
            → Use Arc<Mutex<T>>
            ⭐⭐⭐ Most common
  → No: Use Arc<T> (read-only)
      ⭐⭐ Simplest

Need thread communication?
  → Yes:
      → Simple message passing?
         → Use mpsc::channel
         ⭐⭐ Most idiomatic
      → Complex coordination?
         → Use Condvar
         ⭐⭐⭐ More power, more complexity

Need to avoid shared state?
  → Yes: Use channels instead of Arc<Mutex>!
      ⭐⭐ More Rustic approach
```

---

## Summary Table

| Type | Difficulty | Ownership | Thread-Safe | Mutability | When to Use |
|------|-----------|-----------|-----------|-----------|----------|
| `Arc<T>` | ⭐⭐ | Multiple | ✓ Atomic counting | No | Shared read-only |
| `Mutex<T>` | ⭐⭐ | Single | ✓ Lock | Yes (exclusive) | Single thread with shared access |
| `RwLock<T>` | ⭐⭐⭐ | Single | ✓ Read-write lock | Yes (exclusive writes) | Read-heavy workloads |
| `Arc<Mutex<T>>` | ⭐⭐⭐ | Multiple | ✓ Atomic + Lock | Yes (exclusive) | **Shared mutable state** |
| `Arc<RwLock<T>>` | ⭐⭐⭐ | Multiple | ✓ Atomic + RW Lock | Yes (exclusive writes) | Read-heavy mutable shared data |
| `Atomic*` | ⭐⭐⭐⭐ | - | ✓ Lock-free | Yes (Copy only) | High-frequency counters, flags |
| `mpsc::channel` | ⭐⭐ | One sender/receiver | ✓ | Via message | Producer-consumer, decoupling |
| `Condvar` | ⭐⭐⭐ | - | ✓ | Via notify | Thread coordination, wait-notify |

---

## Single-Threaded vs Multi-Threaded Side-by-Side

```rust
// ========== SINGLE-THREADED (Leptos) ==========
use std::rc::Rc;
use std::cell::RefCell;

let state = Rc::new(RefCell::new(vec![1, 2, 3]));
let state2 = Rc::clone(&state);

{
    let mut guard = state.borrow_mut();
    guard.push(4);
}  // Guard dropped, lock released

{
    let mut guard = state2.borrow_mut();
    guard.push(5);
}

println!("{:?}", state.borrow());  // [1, 2, 3, 4, 5]

// ========== MULTI-THREADED (Tokio) ==========
use std::sync::{Arc, Mutex};
use std::thread;

let state = Arc::new(Mutex::new(vec![1, 2, 3]));
let state2 = Arc::clone(&state);

thread::spawn({
    let state = Arc::clone(&state);
    move || {
        let mut guard = state.lock().unwrap();
        guard.push(4);
    }  // Guard dropped, lock released
}).join().unwrap();

thread::spawn({
    let state = Arc::clone(&state2);
    move || {
        let mut guard = state.lock().unwrap();
        guard.push(5);
    }
}).join().unwrap();

println!("{:?}", state.lock().unwrap());  // [1, 2, 3, 4, 5]
```

---

## Performance Comparison

| Operation | Cost | Notes |
|-----------|------|-------|
| `Rc::clone()` | 1-2 CPU cycles | Non-atomic increment |
| `Arc::clone()` | 5-10 CPU cycles | Atomic increment (synchronization) |
| `RefCell::borrow()` | 1-2 CPU cycles | Runtime counter check |
| `Mutex::lock()` | 50-500 CPU cycles | OS syscall, context switch possible |
| `RwLock::read()` | 20-100 CPU cycles | Atomic compare-and-swap |
| `RwLock::write()` | 50-500 CPU cycles | Exclusive lock, slower |
| `Atomic::load()` | 1-5 CPU cycles | Hardware atomic, very fast |

**Takeaway:** Use `Arc<RwLock>` for frequent reads, `Arc<Mutex>` for balanced, `Atomic*` only for counters.

---

## Real-World: Single-threaded vs Multi-threaded

```rust
// ===== LEPTOS (Single-threaded UI) =====
// In your project:
pub struct OptimizerStateSignals {
    pub parameters: RwSignal<Vec<f32>>,  // Internally: Rc<RefCell<Vec<f32>>>
}

// Ultra-fast, zero locks, perfect for UI reactivity
state.parameters.set(new_vec);

// ===== TOKIO SERVER (Multi-threaded) =====
// Hypothetical async server:
use std::sync::Arc;
use std::sync::RwLock;

pub struct SharedState {
    pub users: Arc<RwLock<Vec<User>>>,  // Many threads reading user list
    pub config: Arc<Config>,            // Read-only config
}

// Multiple request handlers access simultaneously
let users = state.users.read().unwrap();
for user in users.iter() { ... }
```
