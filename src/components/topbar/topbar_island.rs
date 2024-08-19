use icondata::{AiHomeOutlined, BiBookAddRegular, BsFileEarmarkPerson, CgWorkAlt, LuContact};
use leptos::*;
use leptos_icons::Icon;

#[island]
pub fn Topbar() -> impl IntoView {
    let (active_nav, set_active_nav) = create_signal("#home");

    view! {
        <nav>
            <a href="#home" on:click=move |_| set_active_nav("#home") class:active={move || active_nav.get() == "#home"}>
                <Icon icon=AiHomeOutlined width="2em" height="2em"/>
            </a>
            <a href="#about" on:click=move |_| set_active_nav("#about") class:active={move || active_nav.get() == "#about"}>
                <Icon icon=BsFileEarmarkPerson width="2em" height="2em"/>
            </a>
            <a href="#experience" on:click=move |_| set_active_nav("#experience") class:active={move || active_nav.get() == "#experience"}>
                <Icon icon=BiBookAddRegular width="2em" height="2em"/>
            </a>
            <a href="#portfolio" on:click=move |_| set_active_nav("#portfolio") class:active={move || active_nav.get() == "#portfolio"}>
                <Icon icon=CgWorkAlt width="2em" height="2em"/>
            </a>
            <a href="#contact" on:click=move |_| set_active_nav("#contact") class:active={move || active_nav.get() == "#contact"}>
                <Icon icon=LuContact width="2em" height="2em"/>
            </a>
        </nav>
    }
}
