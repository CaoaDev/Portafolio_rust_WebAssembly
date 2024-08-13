use leptos::*;

#[component]
pub fn Testimonials() -> impl IntoView {
    // Definimos la estructura del testimonio
    struct Testimonial {
        id: u32,
        link: &'static str,
        name: &'static str,
        role: &'static str,
        test: &'static str,
    }

    // Datos de los testimonios
    let testimonials = vec![
        Testimonial {
            id: 1,
            link: "https://www.linkedin.com/in/uzairali19/",
            name: "Uzair Ali",
            role: "Full-Stack Web Developer. JavaScript | Rails | React | Redux. Improving open-source projects, one commit at a time.",
            test: "As a developer and a problem solver, I think Meri is a great collaborative partner to have. I met Meri in some basic javascript & react projects and since then she has drastically progressed in her understanding of the development process. She always has a professional environment and has good audio and video quality which makes it easier to communicate with her.",
        },
        Testimonial {
            id: 2,
            link: "https://www.linkedin.com/in/yishak-wesego/",
            name: "Yishak Wesego",
            role: "Full-stack developer | Technical Support Engineer at Microverse",
            test: "It was a pleasure collaborating with Meri on different projects. One of the things that I think is special about her is that she never settles, even after completing the projects she finds other resources and strengthens her knowledge. Collaborating with her is easy and comfortable, it's like working with someone you've known for a long period of time.",
        },
        Testimonial {
            id: 3,
            link: "https://www.linkedin.com/in/akuu-khan/",
            name: "akbar (Aku) Khan",
            role: "Full Stack Developer| Ruby on Rails | PostgreSQL | JavaScript | React | Redux | Html&Css | Python.",
            test: "I worked with Meri in the same team and her communication skills are very strong. Her programming skill is one of the bests, given the time frame in the field. She is a good team player.  She will probably fit in most of the companies not only because she's a good team worker, but also because she has very good skills and I know she has much more potential to be shown.",
        },
        Testimonial {
            id: 4,
            link: "https://www.linkedin.com/in/isaicespedes/",
            name: "Isai Céspedes",
            role: "Full-Stack Web Developer. JavaScript | Rails | React | Redux.",
            test: "I mentored Meri some months ago, and I can say that she is one of those people that you love to work with. She was always interested in what I was trying to teach her, paying attention and always asking questions if something was not clear. When it comes to professional skills, she is really committed to work, always doing her best and going beyond the requirements of the project she's building.",
        },
        Testimonial {
            id: 5,
            link: "https://www.linkedin.com/in/hamzaalitarar/",
            name: "Hamza Tarar",
            role: "Software Developer",
            test: "Throughout all our collaborations, Meri has always conducted herself politely and kindly. She comes across as someone that's always willing to help and puts the team ahead of herself. But beneath this, I see a strength and determination to distinguish herself. She's not only someone I highly recommend but is also someone I greatly respect.",
        },
        Testimonial {
            id: 6,
            link: "https://www.linkedin.com/in/rex9/",
            name: "Htet (Rex) Naing",
            role: "Full Stack Developer | Mindful Mentor | Cheerful Team Player",
            test: "Meri is really cheerful and supportive person. I know her when she reviewed my resume. Her feedback was so crystal clear and super effective for me. After that, even though she has no responsibility of reviewing my resume again. She helped me patiently when I ask her again and again. Meri is gifted in reviewing the work and giving advice to others. I feel really lucky to have her as the reviewer of my resume.",
        },
    ];

    // Renderiza el componente
    view! {
        <section id="testimonials">
            <h5>"Feedback from my peers"</h5>
            <h2>"Testimonials"</h2>
            <div class="container testimonials__container">
                {testimonials.iter().map(|test| view! {
                    <div class="testimonial" key={test.id}>
                        <div class="client__avatar">
                            <a href={test.link} target="_blank" rel="noopener noreferrer">
                                // <img src="ruta/a/tu/icono/linkedin.png" alt="LinkedIn" />  <!-- Cambia esta línea si tienes una imagen del ícono -->
                            </a>
                        </div>
                        <h5 class="client__name">{test.name}</h5>
                        <small class="client__review">{test.test}</small>
                    </div>
                }).collect::<Vec<_>>()}
            </div>
        </section>
    }
}
