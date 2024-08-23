use leptos::{prelude::*, reactive_graph::wrappers::write::SignalSetter};
use leptos_meta::{provide_meta_context, MetaTags};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    path,
};
use thaw::*;

use crate::app::pages::*;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html> 
        <html>
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <ConfigProvider>
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
                loading_bar.start()
            } else {
                loading_bar.finish()
            }
        },
        false,
    );

    view! {
        <Router set_is_routing>
            <Routes fallback=|| "404">
                <Route path=path!("/") view=HomePage />
            </Routes>
        </Router>
    }
}
