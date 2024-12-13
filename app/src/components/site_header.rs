use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use thaw::*;

// Initalize, HomeButton
#[component]
pub fn SiteHeader() -> impl IntoView {
    let navigate = use_navigate();

    view! {
        <LayoutHeader class="demo-header">
            <Space on:click=move |_| {
                navigate("/", Default::default());
            }>
                <img src="/logo.svg" style="width: 36px" />
                <div class="demo-name">"Thaw UI"</div>
            </Space>
        </LayoutHeader>
    }
}
