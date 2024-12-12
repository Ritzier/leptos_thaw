use leptos::prelude::*;
use thaw::*;

#[component]
pub fn SwitchVersion() -> impl IntoView {
    let options = vec![("main", "https://thawui.vercel.app")];

    let version = RwSignal::new(None::<String>);
    let label = RwSignal::new(String::new());
    #[cfg(any(feature = "hydrate"))]
    {
        let location = window().location();
        let origin = location.origin().ok();
        if let Some(origin) = origin.as_ref() {
            if let Some(item) = options.iter().find(|item| item.1 == origin) {
                label.set(item.0.to_string());
            }
        }
        version.set(origin);
        let _ = version.watch(move |origin| {
            if let Some(origin) = origin {
                let pathname = location.pathname().unwrap_or_default();
                let href = format!("{}{}", origin, pathname);
                let _ = location.set_href(&href);
            }
        });
    }

    view! {
        <Combobox value=label selected_options=version placeholder="Switch version">
            {options
                .into_iter()
                .map(|option| view! { <ComboboxOption value=option.1 text=option.0 /> })
                .collect_view()}
        </Combobox>
    }
}
