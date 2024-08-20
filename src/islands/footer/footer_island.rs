use leptos::*;

#[island]
pub fn Footer() -> impl IntoView {
    // Año fijo en lugar de calcularlo dinámicamente
    let current_year = "2024";

    view! {
        <footer>
            <a href="#home" class="footer__logo">"CaoaDev"</a>
            <ul class="permalinks">
                <li><a href="#home">"Home"</a></li>
                <li><a href="#about">"About"</a></li>
                <li><a href="#experience">"Skills"</a></li>
                <li><a href="#portfolio">"Portfolio"</a></li>
                // <li><a href="#testimonials">"Testimonials"</a></li>
                <li><a href="#contact">"Contact"</a></li>
            </ul>
            <div class="footer__socials">
                <a href="www.linkedin.com/in/carlos-a-ortiz-avelino-954713232/" target="_blank" rel="noreferrer"><BsLinkedin/></a>
                <a href="https://github.com/CaoaDev/" target="_blank" rel="noreferrer"><FaGithub/></a>
                // <a href="https://angel.co/u/meri-gogichashvili" target="_blank" rel="noreferrer"><FaAngellist/></a>
            </div>
            <div class="footer__copyright">
                <small>"© ET " {current_year} ". All rights reserved."</small>
            </div>
        </footer>
    }
}

#[island]
fn BsLinkedin() -> impl IntoView {
    view! { <span>"LinkedIn"</span> }
}

#[island]
fn FaGithub() -> impl IntoView {
    view! { <span>"GitHub"</span> }
}

// #[island]
// fn FaAngellist() -> impl IntoView {
//     view! { <span>"AngelList"</span> }
// }
