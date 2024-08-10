use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header id="home">
            <div class="container header__container">
                <h5>"Hello I'm"</h5>
                <h1>"Carlos A. Ortiz Avelino"</h1>
                <h5 class="text-light">"Full-stack Developer Jr."</h5>
                <CTA />
                <a href="#contact" class="scroll__down">"Scroll Down"</a>
                <HeaderSocials />
            </div>
        </header>
    }
}

#[component]
pub fn HeaderSocials() -> impl IntoView {
    view! {
        <div class="header__socials">
            <a href="https://www.linkedin.com/in/carlos-a-ortiz-avelino-954713232/" target="_blank" rel="noreferrer">
                <BsLinkedin />
            </a>
            <a href="https://github.com/CaoaDev/" target="_blank" rel="noreferrer">
                <FaGithub />
            </a>
            // <a href="https://angel.co/u/meri-gogichashvili" target="_blank" rel="noreferrer">
            //     <FaAngellist />
            // </a>
        </div>
    }
}

#[component]
pub fn CTA() -> impl IntoView {
    view! {
        <div class="cta">
            <a href="public/assets/Curriculum Vitae CAOA, Actual.pdf" download class="btn">
                "Download C.V. Spanish"
            </a>
        </div>
    }
}

#[component]
fn BsLinkedin() -> impl IntoView {
    view! { <span>"LinkedIn"</span> }
}

#[component]
fn FaGithub() -> impl IntoView {
    view! { <span>"GitHub"</span> }
}

// #[component]
// fn FaAngellist() -> impl IntoView {
//     view! { <span>"AngelList"</span> }
// }
