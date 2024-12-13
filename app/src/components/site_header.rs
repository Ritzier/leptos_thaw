use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use thaw::*;

// Initalize, HomeButton
#[component]
pub fn SiteHeader() -> impl IntoView {
    let navigate = use_navigate();

    view! {
        <Style id="demo-header">
            "
            .demo-header {
                height: 64px;
                display: flex;
                align-items: center;
                justify-content: space-between;
                padding: 0 20px;
                z-index: 1000;
                position: relative;
                border-bottom: 1px solid var(--colorNeutralStroke2);
            }
            .demo-name {
                cursor: pointer;
                display: flex;
                align-items: center;
                height: 100%;
                font-weight: 600;
                font-size: 20px;
            }
            .demo-header__menu-mobile {
                display: none !important;
            }
            .demo-header__right-btn .thaw-select {
                width: 60px;
            }
            @media screen and (max-width: 600px) {
                .demo-header {
                    padding: 0 8px;
                }
                .demo-name {
                    display: none;
                }
            }
            @media screen and (max-width: 1200px) {
                .demo-header__right-btn {
                    display: none !important;
                }
                .demo-header__menu-mobile {
                    display: inline-block !important;
                }
            }
            "
        </Style>

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
