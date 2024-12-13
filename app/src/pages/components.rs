use leptos::prelude::*;
use thaw::*;

use crate::components::SiteHeader;

#[component]
pub fn ComponentsPage() -> impl IntoView {
    view! {
        <Layout position=LayoutPosition::Absolute>
            <SiteHeader />
        </Layout>
    }
}
