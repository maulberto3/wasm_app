use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/wasm_app.css"/>
        <Title text="Leptos + Axum Counter"/>

        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <div class="container">
            <header>
                <h1>Welcome to Leptos!</h1>
                <p class="subtitle">A full-stack Rust web framework</p>
            </header>

            <section class="demo">
                <h2>Counter Example</h2>
                <button on:click=on_click>
                    "Click me: " {count}
                </button>
            </section>

            <footer>
                <p>Powered by <code>Leptos</code> + <code>Axum</code> + <code>WASM</code></p>
            </footer>
        </div>
    }
}
