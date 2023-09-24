use leptos::*;
use leptos_meta::*;
// use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (dark_theme, set_dark_theme) = create_signal(true);
    let dark_control = move |ev| set_dark_theme.update(|dark| *dark = event_target_checked(&ev));

    let dark_mode_render = move || {
        if dark_theme() {
            view! {
            <Html attributes={vec![("data-theme", Attribute::String(Oco::Borrowed("dark")))]} />
                }
        } else {
            view! {
            <Html attributes={vec![("data-theme", Attribute::String(Oco::Borrowed("light")))]}/>
            }
        }
    };

    view! {
        {dark_mode_render}

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/rusty-llama.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        <div class="flex gap-4 p-4">
            <div class="flex-none">
                <ul class="menu bg-base-200 w-56 rounded-box">
                    <li class="menu-title">
                        Previous Conversations
                    </li>
                    <li>
                        <a>
                            Item 1
                        </a>
                    </li>
                    <li>
                        <a>
                            Item 2
                        </a>
                    </li>
                    <li>
                        <a>
                            Item 3
                        </a>
                    </li>
                    <li class="form-control">
                        <label class="label cursor-pointer">
                            <span class="label-text font-semibold">
                                Dark mode
                            </span>
                            <input
                                type="checkbox"
                                class="toggle"
                                prop:checked=dark_theme
                                on:change=dark_control
                            />
                        </label>
                    </li>
                </ul>
            </div>
            <div class="grow text-center bg-base-200 rounded-box">
                <Home/>
            </div>
        </div>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { <h1>"Not Found"</h1> }
}

#[component]
fn Home() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <h2 class="p-6 text-4xl">"Welcome to Leptos with Tailwind"</h2>
        <p class="px-10 pb-10">
            "Tailwind will scan your Rust files for Tailwind class names and compile them into a CSS file."
        </p>
        <button
            class="bg-amber-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
            on:click=move |_| set_count.update(|count| *count += 1)
        >
            "Something's here | "
            {move || if count() == 0 { "Click me!".to_string() } else { count().to_string() }}

            " | Some more text"
        </button>
    }
}
