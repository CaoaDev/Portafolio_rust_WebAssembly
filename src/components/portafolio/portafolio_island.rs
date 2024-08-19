use leptos::*;

#[island]
pub fn Portfolio() -> impl IntoView {
    // Definimos la estructura del proyecto
    struct Project {
        id: u32,
        title: &'static str,
        // img: &'static str,
        description: &'static str,
        technologies: &'static str,
        // link: &'static str,
        github: &'static str,
    }

    // Datos de los proyectos
    let solo_projects = vec![
        Project {
            id: 1,
            title: "Palindrome Check - Backend",
            // img: "/path/to/img1.jpg", // Cambia esto a la ruta correcta
            description: "This backend service provides an API endpoint to check if a given string is a palindrome. A palindrome is a string that reads the same forward and backward.",
            technologies: "NestJS",
            // link: "https://adventistlearningcenter-257375ab1970.herokuapp.com/",
            github: "https://github.com/CaoaDev/backend-pruebaPalinPdf",
        },
        Project {
            id: 2,
            title: "Portfolio Rust - WebAssembly",
            // img: "/path/to/img4.jpg", // Cambia esto a la ruta correcta
            description: "This project is a personal portfolio built with Rust and WebAssembly, designed to showcase your work and skills. It includes sections for your professional experience, projects, and contact information, all rendered efficiently in the browser with WebAssembly for a smooth user experience.",
            technologies: "Rust | WebAssembly",
            // link: "https://fakestore-metrics.netlify.app/",
            github: "https://github.com/CaoaDev/Portafolio_rust_WebAssembly",
        },
        Project {
            id: 3,
            title: "To-Do App - Frontend",
            // img: "/path/to/img2.jpg", // Cambia esto a la ruta correcta
            description: "TodoList-Svelte is a Svelte application for managing personal tasks. It enables users to add, edit, and delete tasks with a clean and intuitive interface.",
            technologies: "Svelte | CSS",
            // link: "https://meri-mg.github.io/To-Do-List/dist/",
            github: "https://github.com/CaoaDev/todo-svelte-app",
        },
        Project {
            id: 4,
            title: "RickandMorty - Frontend",
            // img: "/path/to/img3.jpg", // Cambia esto a la ruta correcta
            description: "RickAndMorty Image Gallery is a React app that displays images from the Rick and Morty series as interactive cards. Users can browse through various characters and scenes, with each card featuring details and visuals from the show.",
            technologies: "Svelte | CSS",
            // link: "https://meri-mg.github.io/shelter/pages/main/index.html",
            github: "https://github.com/CaoaDev/RickandMorty-svelte-app",
        },
        Project {
            id: 5,
            title: "Shopping Cart - Frontend",
            // img: "/path/to/img5.jpg", // Cambia esto a la ruta correcta
            description: "React shopping cart application that allows users to add, view, and remove items from their cart. It features a dynamic user interface for managing product quantities and totals, offering a seamless shopping experience.",
            technologies: "React | CSS",
            // link: "https://meri-mg.github.io/Unilab-world-news/",
            github: "https://github.com/CaoaDev/CarritoDeCompras-React",
        },
        Project {
            id: 6,
            title: "Pokecards - Frontend",
            // img: "/path/to/img6.jpg", // Cambia esto a la ruta correcta
            description: "Pokecards is a Next.js application that allows users to search for and view Pokémon cards. It offers a user-friendly interface to browse through a collection of Pokémon cards, showcasing essential details and images for each card.",
            technologies: "NextJS | Scss ",
            // link: "https://lukinoo.github.io/math-resource/",
            github: "https://github.com/CaoaDev/pokemon-next.js",
        },
    ];

    // Renderiza el componente
    view! {
        <section id="portfolio">
            <h5>"My Recent Work"</h5>
            <h2>"Portfolio"</h2>

            <div class="container portfolio__container">
                {solo_projects.iter().map(|pro| view! {
                    <article class="portfolio__item" key={pro.id}>
                        // <div class="portfolio__item-image">
                        //     <img src={pro.img} alt={pro.title} />
                        // </div>
                        <div class="portfolio__item-content">
                            <h3>{pro.title}</h3>
                            <p>{pro.description}</p>
                            <p>{pro.technologies}</p>
                        </div>
                        <div class="portfolio__item-cta">
                            <a
                                href={pro.github}
                                target="_blank"
                                class="btn"
                                rel="noreferrer"
                            >
                                "GitHub"
                            </a>
                            // <a
                            //     href={pro.link}
                            //     target="_blank"
                            //     class="btn btn-primary"
                            //     rel="noreferrer"
                            // >
                            //     "Visit Website"
                            // </a>
                        </div>
                    </article>
                }).collect::<Vec<_>>()}
            </div>
        </section>
    }
}
