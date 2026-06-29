# Rust Smart Pointers & Interior Mutability: Learning Guide

## Key Insight: Why Not Just Use `&` (References)?

**The Problem:**

```rust
// You might think: "Can't I just use & for sharing?"
fn share_data(data: &Vec<i32>) {
    let player = Player { data };  // ❌ Error: lifetime issues
    let enemy = Enemy { data };    // ❌ Compiler: "How long does player live?"
}

// The lifetime problem:
struct Player<'a> {
    data: &'a Vec<i32>,  // ← Tied to some lifetime
}

struct Enemy<'a> {
    data: &'a Vec<i32>,  // ← Same lifetime
}

// Who owns the original Vec? How long must it live?
// The compiler demands answers for every reference!
```

**Why this fails:**

- `&` requires explicit lifetimes: `&'a`, `&'b`, etc.
- Multiple structs need multiple lifetime parameters
- If data lives on stack, it's deallocated when scope ends
- All refs become invalid simultaneously
- Code becomes unreadable: `struct<'a, 'b, 'c, 'd>`

**`Rc` solves this:**

```rust
// No lifetimes needed!
struct Player {
    data: Rc<Vec<i32>>,  // ← No 'a, 'b, 'c...
}

struct Enemy {
    data: Rc<Vec<i32>>,  // ← Simple!
}

// Rc dynamically keeps Vec alive as long as ANY Player/Enemy holds it
// When last holder drops, Rc cleans up automatically
```

**The core difference:**

| Aspect | `&` References | `Rc` |
|--------|---|---|
| **Lifetime** | Must be explicit in types | Automatic (ref counting) |
| **Flexibility** | Tied to scope | Lives as long as needed |
| **Stack-based** | ✓ Always on stack | ✗ Points to heap |
| **Complexity** | Simpler code | More overhead (1 extra allocation) |
| **When to use** | Short-lived borrows | Ownership across multiple scopes |

**Practical example:**

```rust
// This won't compile:
let temp = vec![1, 2, 3];
let player = Player { data: &temp };  // ❌ temp dropped at end of scope
                                       // player's reference becomes invalid

// This works:
let data = Rc::new(vec![1, 2, 3]);
let player = Player { data: Rc::clone(&data) };  // ✓ data lives until last Rc dropped
```

---

## 1. `Box<T>` - Stack to Heap (Simplest)

**Implementation Difficulty: ⭐ Easy Win**

**Problem:** Value is too large for stack, or need known size for trait objects.

**Essential Methods:**
- `Box::new(value)` - Create boxed value
- Deref automatically: `*b` or just use like normal value

```rust
// Before: Recursive type won't compile (infinite size)
enum List {
    Node(i32, List),  // ❌ Compiler error: recursive type has infinite size
}

// After: Box makes it finite
enum List {
    Node(i32, Box<List>),  // ✓ Box has fixed size on stack
    Nil,
}

let list = List::Node(1, Box::new(List::Node(2, Box::new(List::Nil))));
```

**Takeaway:** Single owner, move semantics. Once moved, original ref is invalid.

```rust
let b = Box::new(5);
let b2 = b;  // ownership transferred
println!("{}", b);  // ❌ Error: b moved

// Automatic deref:
let b = Box::new(vec![1, 2, 3]);
println!("{}", b.len());  // ✓ Works! Deref coercion handles it
```

**Real-world:** Used in recursive types, large stack allocations, type erasure (`Box<dyn Trait>`)

---

## 2. `Rc<T>` - Multiple Owners (No Mutation)

**Implementation Difficulty: ⭐⭐ Easy**

**Problem:** Multiple parts of code need to own the same data (read-only).

**Essential Methods:**
- `Rc::new(value)` - Create new reference-counted pointer
- `Rc::clone(&rc)` - Increment ref count (cheap, not deep clone)
- `Rc::strong_count(&rc)` - Check how many owners
- `.deref()` automatic: use `rc.field` directly

```rust
// Scenario: Game with entities that reference a shared asset

struct Asset {
    id: u32,
    data: Vec<u8>,
}

struct Player {
    name: String,
    texture: Rc<Asset>,  // Shared, won't duplicate asset in memory
}

struct Enemy {
    health: i32,
    texture: Rc<Asset>,  // Same asset, ref count increments
}

let asset = Rc::new(Asset { 
    id: 1, 
    data: vec![/* texture data */] 
});

println!("Ref count: {}", Rc::strong_count(&asset));  // 1

let player = Player {
    name: "Hero".to_string(),
    texture: Rc::clone(&asset),  // Rc count = 2
};

println!("Ref count: {}", Rc::strong_count(&asset));  // 2

let enemy = Enemy {
    health: 100,
    texture: Rc::clone(&asset),  // Rc count = 3
};

println!("Ref count: {}", Rc::strong_count(&asset));  // 3

// When player dropped: count = 2
// When enemy dropped: count = 1  
// When asset dropped (last ref): cleaned up automatically
```

**Key:** No interior mutability! Can't call `.set()` or mutate through `Rc`.

```rust
// Won't compile:
rc.value = 42;  // ❌ Rc<T> doesn't give &mut
rc.data.push(item);  // ❌ Can't mutate immutably

// Solution: Wrap in RefCell
let rc = Rc::new(RefCell::new(value));
rc.borrow_mut().data.push(item);  // ✓ Works!
```

---

## 3. `Cell<T>` - Interior Mutability (Single-threaded, Copy)

**Implementation Difficulty: ⭐ Easy Win**

**Problem:** Need to mutate small values without `&mut`.

**Essential Methods:**
- `Cell::new(value)` - Create cell
- `.get()` - Get copy of value (only works for Copy types!)
- `.set(value)` - Replace value (consumes new value)
- `.replace(value)` - Replace and return old value

```rust
// Scenario: Counter that tracks how many times it's been read (without &mut)

struct Counter {
    value: i32,
    read_count: Cell<i32>,  // Track reads without &mut self
}

impl Counter {
    fn get(&self) -> i32 {  // ← Only &self, not &mut self!
        let count = self.read_count.get();
        self.read_count.set(count + 1);  // Interior mutability magic
        self.value
    }
    
    fn reset_count(&self) {
        self.read_count.set(0);  // Easy!
    }
}

let counter = Counter { value: 42, read_count: Cell::new(0) };
println!("{}", counter.get());  // Increments read_count
println!("{}", counter.get());  // Increments again
println!("Reads: {}", counter.read_count.get());  // Output: Reads: 2
```

**Limitation:** Only works for `Copy` types!

```rust
// Won't compile:
let cell = Cell::new(String::from("hello"));
cell.set(String::from("world"));  // ❌ Error: String isn't Copy
                                   // Need RefCell instead!

// Works fine:
let cell = Cell::new(5);
println!("{}", cell.get());  // ✓ i32 is Copy
```

**When to use:** Cached values, counters, flags—anything small and Copy.

---

## 4. `RefCell<T>` - Interior Mutability (Non-Copy Types)

**Implementation Difficulty: ⭐⭐ Easy (but worth understanding panics!)**

**Problem:** Need to mutate non-Copy types without `&mut`.

**Essential Methods:**
- `RefCell::new(value)` - Create cell
- `.borrow()` - Get immutable reference (panics if already mutably borrowed)
- `.borrow_mut()` - Get mutable reference (panics if already borrowed)
- `.try_borrow()` - Safe version, returns `Result`
- `.try_borrow_mut()` - Safe version, returns `Result`

```rust
// Scenario: UI component that updates its state without &mut

struct TextBox {
    label: String,
    content: RefCell<String>,  // Can mutate String inside
}

impl TextBox {
    fn update_content(&self, new_text: &str) {  // ← Only &self!
        self.content.borrow_mut().push_str(new_text);  // Interior mutability
    }

    fn get_content(&self) -> String {
        self.content.borrow().clone()
    }
}

let textbox = TextBox {
    label: "Search".to_string(),
    content: RefCell::new(String::new()),
};

textbox.update_content("hello");
textbox.update_content(" world");
println!("{}", textbox.get_content());  // Output: hello world
```

**Runtime borrow checking (can PANIC!):**

```rust
let content = RefCell::new(String::from("test"));

let borrow1 = content.borrow();       // ✓ Immutable borrow
let borrow2 = content.borrow();       // ✓ Another immutable borrow
println!("{:?}", borrow1);            // ✓ Works!

// This would PANIC:
// let mut borrow3 = content.borrow_mut();  // ❌ Panic! Already borrowed

drop(borrow1);  // Release borrows
drop(borrow2);

let mut borrow3 = content.borrow_mut();  // ✓ Now works!
borrow3.push_str("!");
```

**Safe alternative (no panics):**

```rust
let content = RefCell::new(String::from("test"));

match content.try_borrow_mut() {
    Ok(mut guard) => guard.push_str("!"),
    Err(_) => println!("Already borrowed!"),
}
```

**When to use:** State that changes without `&mut` (UI, caches, internal tracking).

---

## 5. COMPOSITE: `Rc<RefCell<T>>` - Shared Mutable State

**Implementation Difficulty: ⭐⭐⭐ Medium (very common pattern)**

**This is what your Leptos signals use! 🎯**

**Essential Methods:**
- Create: `Rc::new(RefCell::new(value))`
- Share: `Rc::clone(&rc_ref_cell)`
- Read: `rc_ref_cell.borrow().field`
- Write: `rc_ref_cell.borrow_mut().field = value`

**Problem:** Multiple owners need to mutate the same data.

```rust
use std::rc::Rc;
use std::cell::RefCell;

// Scenario: Game entities that share a damage counter

struct GameState {
    damage: i32,
}

struct Player {
    name: String,
    state: Rc<RefCell<GameState>>,  // ← Shared mutable state!
}

struct Enemy {
    health: i32,
    state: Rc<RefCell<GameState>>,  // ← Same state
}

let game_state = Rc::new(RefCell::new(GameState { damage: 0 }));

let player = Player {
    name: "Hero".to_string(),
    state: Rc::clone(&game_state),  // Rc count = 2
};

let enemy = Enemy {
    health: 100,
    state: Rc::clone(&game_state),  // Rc count = 3
};

// Player and Enemy can both mutate the shared state
player.state.borrow_mut().damage += 10;
println!("Damage: {}", enemy.state.borrow().damage);  // Output: 10

// No data races! RefCell ensures single mutable borrow at a time
```

---

## 6. COMPOSITE: `Rc<RefCell<Vec<T>>>` - Shared Growing Collection

**Implementation Difficulty: ⭐⭐ Easy (very common pattern)**

**Problem:** Multiple components add items to a shared list.

```rust
use std::rc::Rc;
use std::cell::RefCell;

struct EventBus {
    subscribers: Rc<RefCell<Vec<String>>>,
}

struct Logger {
    events: Rc<RefCell<Vec<String>>>,
}

struct Metrics {
    events: Rc<RefCell<Vec<String>>>,
}

let events = Rc::new(RefCell::new(vec![]));

let logger = Logger {
    events: Rc::clone(&events),
};

let metrics = Metrics {
    events: Rc::clone(&events),
};

// Different components push events
logger.events.borrow_mut().push("User logged in".to_string());
metrics.events.borrow_mut().push("CPU: 45%".to_string());

println!("All events: {:?}", events.borrow());
// Output: All events: ["User logged in", "CPU: 45%"]
```

**In Leptos, this is similar to:**
```rust
pub struct OptimizerStateSignals {
    pub parameters: RwSignal<Vec<f32>>,  // ← Internally Rc<RefCell<Vec<f32>>>
    pub best_individual_idx: RwSignal<Option<usize>>,
}

// Multiple components access and modify parameters
state.parameters.set(new_vec);  // Writes to shared Vec
```

---

## Real-World Mapping to Your Leptos Project

```rust
// Your code uses:
Rc<RefCell<T>>  ← Inside RwSignal

// This is because:
- Multiple components need to own signals (Rc)
- Components mutate state without &mut (RefCell)
- Leptos dependency tracking happens internally

// You get fine-grained reactivity because:
1. Signal wrapped in Rc<RefCell<>>
2. Leptos tracks which component depends on which signal
3. Only dependents re-render when Rc<RefCell> contents change
```

---

## Summary Table

| Type | Difficulty | Ownership | Interior Mutability | When to Use |
|------|-----------|-----------|-------------------|----------|
| `Box<T>` | ⭐ | Single | No | Move to heap, trait objects, recursive types |
| `Rc<T>` | ⭐⭐ | Multiple | No | Shared read-only data, assets |
| `Cell<T>` | ⭐ | Single | Yes (Copy) | Small copy types that mutate (counters, flags) |
| `RefCell<T>` | ⭐⭐ | Single | Yes (Non-copy) | State without &mut, mutable caches |
| `Rc<RefCell<T>>` | ⭐⭐⭐ | Multiple | Yes | **Shared mutable state (Leptos signals!)** |
| `Rc<RefCell<Vec<T>>>` | ⭐⭐ | Multiple | Yes | Shared growing collections |
| `Rc<RefCell<HashMap<...>>>` | ⭐⭐⭐ | Multiple | Yes | Component registries, dynamic lookup |
