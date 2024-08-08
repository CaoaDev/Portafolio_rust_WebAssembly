use icondata::{OcProjectSm, FaRustBrands};
use leptos::*;
use leptos_icons::Icon;

#[component]
pub fn Intro() -> impl IntoView {
    view! {
        <section id="about">
            <h5>"Get to know"</h5>
            <h2>"About Me"</h2>
            <div class="container about__container">
                <div class="about__me">
                    <div class="about__me-image">
                        <img src="/path/to/me.jpg" alt="me"/>
                    </div>
                </div>
                <div class="about__content">
                    <div class="about__cards">
                        <article class="about__card">
                            <a href="#experience">
                                <Icon icon=FaRustBrands class="about__icon" />
                                <h5>"Experience"</h5>
                                <small>"1+ year"</small>
                            </a>
                        </article>
                        <article class="about__card">
                            <a href="#portfolio">
                                <Icon icon=OcProjectSm class="about__icon" />
                                <h5>"Projects"</h5>
                                <small>"20+ Completed Projects"</small>
                            </a>
                        </article>
                    </div>
                    <p>
                        "I am a Computer Systems Engineer with a solid foundation in both backend and frontend programming. Additionally, I have experience in preventive and corrective support for personal computers. I have conducted training sessions on computer science and the use of computing systems. I hold a master's degree in Telecommunications and am recognized for my problem-solving skills within my area of expertise. My knowledge includes operating systems such as Windows, GNU/Linux (with over 10 years of experience), and to a lesser extent, macOS. I have strong communication skills and strive to approach and solve problems in a self-taught manner."
                    </p>
                    <a href="#contact" class="btn btn-primary">"Let's Talk"</a>
                </div>
            </div>
        </section>
    }
}

#[component]
fn FaAward() -> impl IntoView {
    view! { <span>"Award Icon"</span> }
}

#[component]
fn VscFolderLibrary() -> impl IntoView {
    view! { <span>"Folder Library Icon"</span> }
}
