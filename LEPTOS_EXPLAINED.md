# How Leptos, Rust, WASM, and JavaScript Work Together

A friendly guide for newcomers to full-stack Rust web development.

## The Big Picture

This project is a **full-stack Rust application**. The same Rust code (`src/app.rs`) is compiled into TWO different things:

```
src/app.rs (your component code)
    ↓
    ├─→ [Compile with "ssr" feature]  →  Server binary (x86_64)
    │   └─→ Runs on your server
    │       Renders HTML on the server
    │
    └─→ [Compile with "hydrate" feature]  →  WebAssembly module
        └─→ Downloads to browser
            Makes HTML interactive
```

This is called **Server-Side Rendering (SSR)** with **Client-Side Hydration**.

---

## When You Run `make dev`

```bash
make dev
```

This triggers `cargo leptos watch`, which:

1. **Compiles the server** from `src/main.rs` with SSR feature
   - Creates: `./target/debug/wasm_app` (native binary)
   - This is a full Axum web server

2. **Compiles the client** from `src/lib.rs` with hydrate feature
   - Creates: `./target/site/pkg/wasm_app.wasm` (WebAssembly)
   - Creates: `./target/site/pkg/wasm_app.js` (glue code)
   - This is the interactive browser code

3. **Starts the server** on `http://localhost:3000`
   - The server process runs continuously
   - Watches for file changes
   - Auto-recompiles and hot-reloads browser

---

## What Happens When You Visit `http://localhost:3000`

### Step 1: Browser Requests HTML

```
You visit http://localhost:3000
          ↓
Browser makes HTTP GET request to server
          ↓
Server (./target/debug/wasm_app) receives request
```

### Step 2: Server Renders HTML (SSR)

In `src/app.rs`, the `shell()` function runs:

```rust
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html>
            <head>
                <HydrationScripts options/>  // ← Injects script tags
                <MetaTags/>
            </head>
            <body>
                <App/>  // ← Rendered to static HTML
            </body>
        </html>
    }
}
```

The server:
- Compiles your `App` component to **static HTML strings**
- Example: `<button>Click me: 0</button>`
- Includes script references: `<script src="/pkg/wasm_app.js"></script>`

### Step 3: Browser Receives HTML

The browser gets back:

```html
<!DOCTYPE html>
<html>
  <head>
    <script src="/pkg/wasm_app.js"></script>
    <link rel="stylesheet" href="/pkg/wasm_app.css">
  </head>
  <body>
    <div class="container">
      <button>Click me: 0</button>
    </div>
  </body>
</html>
```

**Important:** At this moment, the button is **NOT interactive**. It's just static HTML.

### Step 4: Browser Sees the UI

Your page appears on screen with:
- ✅ HTML structure rendered
- ✅ CSS styling applied
- ❌ JavaScript not loaded yet
- ❌ No interactivity yet

---

## WASM Enters the Picture

The browser now loads the JavaScript:

```javascript
// Browser downloads and executes: /pkg/wasm_app.js
// This JavaScript file:
// 1. Loads the WebAssembly module (wasm_app.wasm)
// 2. Instantiates it
// 3. Exports Rust functions to JavaScript
// 4. Calls the hydrate() function
```

In `src/lib.rs`:

```rust
#[cfg(feature = "hydrate")]
#[wasm_bindgen]
pub fn hydrate() {
    use crate::app::App;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);  // ← Activates WASM
}
```

**What "hydration" means:**
- The WASM module "attaches itself" to the existing HTML
- It adds event listeners to buttons, inputs, etc.
- It takes over managing the interactive state

**Now the button is interactive!**

---

## What Happens When You Click the Button?

In `src/app.rs`:

```rust
fn HomePage() -> impl IntoView {
    let count = RwSignal::new(0);  // ← WASM manages this
    let on_click = move |_| *count.write() += 1;

    view! {
        <button on:click=on_click>
            "Click me: " {count}
        </button>
    }
}
```

### Click Flow:

```
1. User clicks button
   ↓
2. WASM event handler executes: on_click()
   ↓
3. Signal updates: count changes from 0 to 1
   ↓
4. WASM updates the DOM: "Click me: 1"
   ↓
5. Browser renders the change (instant, no server call!)
```

**Important:** ALL subsequent clicks run entirely in WASM in the browser. The server is not involved.

---

## File Locations & What Each Does

| File | Purpose | When Used |
|------|---------|-----------|
| `src/app.rs` | Shared component code | Both server & browser |
| `src/main.rs` | Server entry point | Server only (SSR) |
| `src/lib.rs` | Browser entry point | Browser only (hydrate) |
| `style/main.css` | Styling | Both (served by server) |
| `target/debug/wasm_app` | Server binary | Server executable |
| `target/site/pkg/wasm_app.wasm` | WebAssembly module | Browser downloads this |
| `target/site/pkg/wasm_app.js` | JavaScript glue | Browser downloads this |
| `target/site/pkg/wasm_app.css` | Compiled CSS | Browser downloads this |

---

## What Are .rlib and .rmeta Files?

These are **intermediate build artifacts** created during compilation:

- `.rlib` = Rust library (intermediate, for linking)
- `.rmeta` = Metadata (type info for the compiler)

They live in `target/debug/deps/` and are **not deployed**. They're just temporary files the Rust compiler uses to link everything together. You can ignore them.

---

## The Size Breakdown

When you run `make build-release`:

- **Server binary:** ~15MB (optimized native code)
- **WASM module:** ~500KB (optimized WebAssembly)
- **CSS:** ~50KB
- **Total deployed:** ~15.5MB

Compare to a JavaScript app with Node.js:
- Node.js runtime: ~100MB
- Your application: ~5MB
- Total: ~105MB

Rust gives you a much smaller footprint while being faster and safer.

---

## The Communication Flow

```
┌─────────────────────────────────────────────────────────┐
│ Initial Page Load                                       │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Browser → GET / → Server                             │
│  Server (in /src/main.rs):                            │
│    - Runs App component (SSR feature)                 │
│    - Generates: <button>Click me: 0</button>          │
│    - Adds: <script src="/pkg/wasm_app.js"></script>   │
│  Server → HTML response → Browser                     │
│                                                         │
│  Browser renders HTML (not interactive yet)            │
│  Browser downloads: wasm_app.js + wasm_app.wasm       │
│  JavaScript runs: hydrate_body(App)                   │
│  WASM is now active and manages the page              │
│                                                         │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│ Subsequent Interactions (e.g., click)                  │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Browser (in /src/app.rs with hydrate feature):      │
│    - WASM event handler fires                         │
│    - Updates signal: count += 1                       │
│    - WASM updates DOM: <button>Click me: 1</button>  │
│    - NO server involved                               │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

## Key Concepts

### Server-Side Rendering (SSR)
- Your Rust code runs on the server
- Compiles to a native binary (`src/main.rs` with "ssr" feature)
- Generates HTML strings before sending to browser
- **Benefit:** Fast initial page load, SEO-friendly

### Hydration
- The WASM module "attaches" to the server-rendered HTML
- Adds interactivity without re-rendering the page
- **Benefit:** Instant interactivity, no flash of unstyled content

### Signals
- Leptos's reactive state management
- When a signal updates, the UI automatically updates
- They run inside WASM in the browser

---

## The Compiler is Your Friend

The two `Cargo.toml` features tell the compiler how to compile `src/app.rs`:

```toml
[features]
ssr = [
    "dep:axum",
    "leptos/ssr",  # ← Compile for server
    # ...
]
hydrate = [
    "dep:wasm-bindgen",
    "leptos/hydrate",  # ← Compile for browser
    # ...
]
```

- **SSR compilation:** Includes server libraries, excludes WASM
- **Hydrate compilation:** Includes WASM bindings, excludes server code

Same source code, compiled twice, two different targets.

---

## Debugging Tips

### "Why is my button not interactive?"
- Check if WASM loaded: Open DevTools → Network → look for `wasm_app.wasm` and `wasm_app.js`
- Check browser console for JavaScript errors

### "Why is my change not showing?"
- In development: `make dev` auto-rebuilds. Wait a moment for recompilation.
- In production: Did you run `make build-release`?

### "How do I see what HTML the server generated?"
- Right-click → "View Page Source" in your browser
- You'll see the pre-rendered HTML

### "Why is the binary so large in debug mode?"
- Debug symbols are included. Run `make build-release` for optimized size (~15MB instead of 142MB).

---

## Next Steps

- Try modifying `src/app.rs` and watch `make dev` auto-update
- Add more signals and interactive elements
- Explore server functions to communicate between WASM and server
- Read the [Leptos Book](https://leptos.dev/)

Happy coding! 🚀
