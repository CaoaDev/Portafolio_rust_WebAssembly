use leptos::*;

/// 404 Not Found Page
#[island]
pub fn NotFound() -> impl IntoView {
    view! { <h1>"Uh oh!" <br/> "We couldn't find that page!"</h1>  }
}
