use leptos::{ev::MouseEvent, prelude::*};
use leptos_meta::Style;
use leptos_router::hooks::use_navigate;
use thaw::*;

use super::switch_version::SwitchVersion;

// Initalize, HomeButton
#[component]
pub fn SiteHeader() -> impl IntoView {
    let navigate = use_navigate();
    //let navigate_signal = RwSignal::new(use_navigate());
    let dir = ConfigInjection::expect_context().dir;
    let theme = Theme::use_rw_theme();
    let theme_name = Memo::new(move |_| {
        theme.with(|theme| {
            if theme.name == *"light" {
                "Dark".to_string()
            } else {
                "Light".to_string()
            }
        })
    });
    let change_theme = move |_| {
        if theme_name.get_untracked() == "Light" {
            theme.set(Theme::light());
        } else {
            theme.set(Theme::dark());
        }
    };

    // TODO: search

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
            // Home button
            <Space on:click=move |_| {
                navigate("/", Default::default());
            }>
                <img src="/logo.svg" style="width: 36px" />
                <div class="demo-name">"Thaw UI"</div>
            </Space>

            <Space align=SpaceAlign::Center>
                <Menu
                    position=MenuPosition::BottomEnd
                    on_select=move |value: String| match value.as_str() {
                        "Dark" => change_theme(MouseEvent::new("click").unwrap()),
                        "Light" => change_theme(MouseEvent::new("click").unwrap()),
                        "github" => _ = window().open_with_url("https://github.com/thaw-ui/thw"),
                        "discord" => {
                            _ = window()
                                .open_with_url(
                                    "https://discord.com/channels/1031524867910148188/1270735289437913108",
                                );
                        }
                        _ => {}
                    }
                >
                    <MenuTrigger slot>
                        <Button
                            class="demo-header__menu-mobile"
                            appearance=ButtonAppearance::Subtle
                            icon=icondata::AiUnorderedListOutlined
                            attr:style="font-size: 22px; padding: 0px 6px;"
                        />
                    </MenuTrigger>
                    <MenuItem value=theme_name>{theme_name}</MenuItem>
                    <MenuItem icon=icondata::AiGithubOutlined value="github">
                        "Github"
                    </MenuItem>
                    <MenuItem icon=icondata::BiDiscordAlt value="discord">
                        "Discord"
                    </MenuItem>
                // TODO: nav
                </Menu>

                <Space class="demo-header__right-btn" align=SpaceAlign::Center>
                    // Switch Version
                    <SwitchVersion />
                    // Theme toggle
                    <Button icon=Memo::new(move |_| {
                        theme
                            .with(|theme| {
                                if theme.name == "light" {
                                    icondata::BiMoonRegular
                                } else {
                                    icondata::BiSunRegular
                                }
                            })
                    })>{move || theme_name.get()}</Button>
                    // Toggle Ltr / Rtl
                    <Button on_click=move |_| {
                        let Some(dir) = dir else {
                            return;
                        };
                        dir.update(move |dir| {
                            *dir = match dir {
                                ConfigDirection::Auto => ConfigDirection::Ltr,
                                ConfigDirection::Ltr => ConfigDirection::Rtl,
                                ConfigDirection::Rtl => ConfigDirection::Ltr,
                            }
                        })
                    }>
                        {move || {
                            let Some(dir) = dir else { return None };
                            match dir.get() {
                                ConfigDirection::Auto => Some("Auto"),
                                ConfigDirection::Ltr => Some("LTR"),
                                ConfigDirection::Rtl => Some("RTL"),
                            }
                        }}
                    </Button>
                </Space>
            </Space>
        </LayoutHeader>
    }
}
