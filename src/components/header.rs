use leptos::*;
use std::fs;

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
    // Define possible paths for the CV
    let file_name = "Curriculum Vitae CAOA, Actual.pdf";
    let paths = [
        format!("public/assets/{}", file_name),
        format!("dist/{}", file_name),
        format!("/{}", file_name),
    ];

    // Function to find a valid path for the CV
    fn find_valid_path(paths: &[String]) -> String {
        for path in paths {
            if fs::metadata(path).is_ok() {
                return path.clone();
            }
        }
        // Return a fallback path if no valid path is found
        "/".to_string()
    }

    let valid_path = find_valid_path(&paths);

    view! {
        <div class="cta">
            <a id="cv-link" href={valid_path} download class="btn">
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
