use leptos::*;

use crate::components::contact::Contact;
use crate::components::experience::Experience;
use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::intro::Intro;
use crate::components::portafolio::Portfolio;
// use crate::components::testimonials::Testimonials;
use crate::components::topbar::Topbar;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <div class="container">
                <Header />
                <Topbar />
                <Intro />
                <Experience />
                <Portfolio />
                // <Testimonials />
                <Contact />
                <Footer />
            </div>
        </ErrorBoundary>
    }
}
