use leptos::{prelude::*, reactive::wrappers::write::SignalSetter};
use leptos_meta::{provide_meta_context, MetaTags};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use thaw::ssr::SSRMountStyleProvider;
use thaw::*;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <SSRMountStyleProvider>
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="utf-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1" />
                    <link rel="shortcut icon" href="/favicon.ico" type="image/x-icon" />
                    <AutoReload options=options.clone() />
                    <HydrationScripts options />
                    <MetaTags />
                </head>
                <body>
                    <App />
                </body>
            </html>
        </SSRMountStyleProvider>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let dir = RwSignal::new(ConfigDirection::Ltr);

    view! {
        <ConfigProvider dir=dir>
            <ToasterProvider>
                <LoadingBarProvider>
                    <TheRouter />
                </LoadingBarProvider>
            </ToasterProvider>
        </ConfigProvider>
    }
}

#[component]
fn TheRouter() -> impl IntoView {
    let loading_bar = LoadingBarInjection::expect_use();
    let is_routing = RwSignal::new(false);
    let set_is_routing = SignalSetter::map(move |is_routing_data| {
        is_routing.set(is_routing_data);
    });

    Effect::watch(
        move || is_routing.get(),
        move |is_routing, _, _| {
            if *is_routing {
                loading_bar.start();
            } else {
                loading_bar.finish();
            }
        },
        false,
    );

    view! {
        <Router set_is_routing>
            <Routes fallback=|| "404">
                <Route path=path!("/") view=Home />
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    let (count, set_count) = signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: "{count}</button>
    }
}
