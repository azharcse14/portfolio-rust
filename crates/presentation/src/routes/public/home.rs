use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text="Azharul Islam — Portfolio" />
        <div id="colorlib-page">
            <div class="container-wrap">
                <a
                    href="#"
                    class="js-colorlib-nav-toggle colorlib-nav-toggle"
                    data-toggle="collapse"
                    data-target="#navbar"
                    aria-expanded="false"
                    aria-controls="navbar"
                ><i></i></a>

                <Aside />

                <div id="colorlib-main">
                    <Hero />
                    <About />
                    <Services />
                    <Counter />
                    <Skills />
                    <Education />
                    <Experience />
                    <Work />
                    <Blog />
                    <Contact />
                </div>
            </div>
        </div>
    }
}

#[component]
fn Aside() -> impl IntoView {
    view! {
        <aside id="colorlib-aside" role="complementary" class="border js-fullheight">
            <div class="text-center">
                <div
                    class="author-img"
                    style="background-image: url(/images/about.jpg);"
                ></div>
                <h1 id="colorlib-logo"><a href="index.html">"Azharul Islam"</a></h1>
                <span class="position">
                    <a href="#">"Full Stack Developer"</a>" in Bangladesh"
                </span>
            </div>
            <nav id="colorlib-main-menu" role="navigation" class="navbar">
                <div id="navbar" class="collapse">
                    <ul>
                        <li class="active"><a href="#" data-nav-section="home">"Home"</a></li>
                        <li><a href="#" data-nav-section="about">"About"</a></li>
                        <li><a href="#" data-nav-section="services">"Services"</a></li>
                        <li><a href="#" data-nav-section="skills">"Skills"</a></li>
                        <li><a href="#" data-nav-section="education">"Education"</a></li>
                        <li><a href="#" data-nav-section="experience">"Experience"</a></li>
                        <li><a href="#" data-nav-section="work">"Work"</a></li>
                        <li><a href="#" data-nav-section="blog">"Blog"</a></li>
                        <li><a href="#" data-nav-section="contact">"Contact"</a></li>
                    </ul>
                </div>
            </nav>

            <div class="colorlib-footer">
                <p><small>
                    "Copyright © 2026 All rights reserved. Made with "
                    <i class="icon-heart" aria-hidden="true"></i>
                    " by Azharul Islam"
                </small></p>
                <ul>
                    <li><a href="#"><i class="icon-facebook2"></i></a></li>
                    <li><a href="#"><i class="icon-twitter2"></i></a></li>
                    <li><a href="#"><i class="icon-instagram"></i></a></li>
                    <li><a href="https://github.com/azharcse14"><i class="icon-linkedin2"></i></a></li>
                </ul>
            </div>
        </aside>
    }
}

#[component]
fn Hero() -> impl IntoView {
    view! {
        <section id="colorlib-hero" class="js-fullheight" data-section="home">
            <div class="flexslider js-fullheight">
                <ul class="slides">
                    <li style="background-image: url(/images/img_bg_1.jpg);">
                        <div class="overlay"></div>
                        <div class="container-fluid">
                            <div class="row">
                                <div class="col-md-6 col-md-offset-3 col-md-pull-3 col-sm-12 col-xs-12 js-fullheight slider-text">
                                    <div class="slider-text-inner js-fullheight">
                                        <div class="desc">
                                            <h1>"Hi! " <br /> "I'm Azharul"</h1>
                                            <h2>"Full Stack Developer building things with Rust, TypeScript, and clean architecture."</h2>
                                            <p>
                                                <a class="btn btn-primary btn-learn">
                                                    "Download CV " <i class="icon-download4"></i>
                                                </a>
                                            </p>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </li>
                    <li style="background-image: url(/images/img_bg_2.jpg);">
                        <div class="overlay"></div>
                        <div class="container-fluid">
                            <div class="row">
                                <div class="col-md-6 col-md-offset-3 col-md-pull-3 col-sm-12 col-xs-12 js-fullheight slider-text">
                                    <div class="slider-text-inner">
                                        <div class="desc">
                                            <h1>"I am " <br /> "a Developer"</h1>
                                            <h2>"Crafting reliable software end-to-end — from database to UI."</h2>
                                            <p>
                                                <a class="btn btn-primary btn-learn">
                                                    "View Portfolio " <i class="icon-briefcase3"></i>
                                                </a>
                                            </p>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </li>
                </ul>
            </div>
        </section>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <section class="colorlib-about" data-section="about">
            <div class="colorlib-narrow-content">
                <div class="row">
                    <div class="col-md-12">
                        <div
                            class="row row-bottom-padded-sm animate-box"
                            data-animate-effect="fadeInLeft"
                        >
                            <div class="col-md-12">
                                <div class="about-desc">
                                    <span class="heading-meta">"About Me"</span>
                                    <h2 class="colorlib-heading">"Who Am I?"</h2>
                                    <p>
                                        <strong>"Hi, I'm Azharul Islam"</strong>
                                        " — a full stack developer who enjoys building well-architected, fast products. "
                                        "I work across the stack with Rust, TypeScript, and the boring-but-important parts of software design."
                                    </p>
                                    <p>
                                        "Currently focused on Rust + Leptos for the web, with a soft spot for clean architecture, "
                                        "fast feedback loops, and software that holds up under maintenance."
                                    </p>
                                </div>
                            </div>
                        </div>
                        <div class="row">
                            <div class="col-md-3 animate-box" data-animate-effect="fadeInLeft">
                                <div class="services color-1">
                                    <span class="icon2"><i class="icon-bulb"></i></span>
                                    <h3>"Product Thinking"</h3>
                                </div>
                            </div>
                            <div class="col-md-3 animate-box" data-animate-effect="fadeInRight">
                                <div class="services color-2">
                                    <span class="icon2"><i class="icon-globe-outline"></i></span>
                                    <h3>"Web Development"</h3>
                                </div>
                            </div>
                            <div class="col-md-3 animate-box" data-animate-effect="fadeInTop">
                                <div class="services color-3">
                                    <span class="icon2"><i class="icon-data"></i></span>
                                    <h3>"Backend / Data"</h3>
                                </div>
                            </div>
                            <div class="col-md-3 animate-box" data-animate-effect="fadeInBottom">
                                <div class="services color-4">
                                    <span class="icon2"><i class="icon-phone3"></i></span>
                                    <h3>"Mobile-friendly"</h3>
                                </div>
                            </div>
                        </div>
                        <div class="row">
                            <div class="col-md-12 animate-box" data-animate-effect="fadeInLeft">
                                <div class="hire">
                                    <h2>"I'm available for freelance work " <br /> "and interesting collaborations!"</h2>
                                    <a href="#" class="btn-hire">"Hire me"</a>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn Services() -> impl IntoView {
    view! {
        <section class="colorlib-services" data-section="services">
            <div class="colorlib-narrow-content">
                <div class="row">
                    <div
                        class="col-md-6 col-md-offset-3 col-md-pull-3 animate-box"
                        data-animate-effect="fadeInLeft"
                    >
                        <span class="heading-meta">"What I do?"</span>
                        <h2 class="colorlib-heading">"Here are some of my expertise"</h2>
                    </div>
                </div>
                <div class="row row-pt-md">
                    <ServiceCard icon="icon-bulb" color="color-1" title="Innovative Ideas"
                        desc="Turning ideas into working software, end-to-end — from sketch to ship." />
                    <ServiceCard icon="icon-data" color="color-2" title="Backend Engineering"
                        desc="Rust, Axum, Postgres, SQLx — type-safe APIs and clean data layers." />
                    <ServiceCard icon="icon-phone3" color="color-3" title="Mobile-friendly UI"
                        desc="Responsive, accessible interfaces that work on every screen size." />
                    <ServiceCard icon="icon-layers2" color="color-4" title="Frontend Craft"
                        desc="Leptos, React, Svelte — modern reactive UIs without the bloat." />
                    <ServiceCard icon="icon-data" color="color-5" title="DevOps Basics"
                        desc="Docker, GitHub Actions, free-tier hosting that just works." />
                    <ServiceCard icon="icon-phone3" color="color-6" title="Maintenance"
                        desc="Code I'll still understand six months from now — and so will you." />
                </div>
            </div>
        </section>
    }
}

#[component]
fn ServiceCard(
    icon: &'static str,
    color: &'static str,
    title: &'static str,
    desc: &'static str,
) -> impl IntoView {
    let services_class = format!("services {}", color);
    let icon_class = icon.to_string();
    view! {
        <div class="col-md-4 text-center animate-box">
            <div class=services_class>
                <span class="icon">
                    <i class=icon_class></i>
                </span>
                <div class="desc">
                    <h3>{title}</h3>
                    <p>{desc}</p>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Counter() -> impl IntoView {
    view! {
        <div
            id="colorlib-counter"
            class="colorlib-counters"
            style="background-image: url(/images/cover_bg_1.jpg);"
            data-stellar-background-ratio="0.5"
        >
            <div class="overlay"></div>
            <div class="colorlib-narrow-content">
                <div class="row"></div>
                <div class="row">
                    <CounterItem to="309" label="Cups of coffee" />
                    <CounterItem to="56" label="Projects" />
                    <CounterItem to="30" label="Clients" />
                    <CounterItem to="10" label="Partners" />
                </div>
            </div>
        </div>
    }
}

#[component]
fn CounterItem(to: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <div class="col-md-3 text-center animate-box">
            <span
                class="colorlib-counter js-counter"
                data-from="0"
                data-to=to
                data-speed="5000"
                data-refresh-interval="50"
            ></span>
            <span class="colorlib-counter-label">{label}</span>
        </div>
    }
}

#[component]
fn Skills() -> impl IntoView {
    view! {
        <section class="colorlib-skills" data-section="skills">
            <div class="colorlib-narrow-content">
                <div class="row">
                    <div
                        class="col-md-6 col-md-offset-3 col-md-pull-3 animate-box"
                        data-animate-effect="fadeInLeft"
                    >
                        <span class="heading-meta">"My Specialty"</span>
                        <h2 class="colorlib-heading animate-box">"My Skills"</h2>
                    </div>
                </div>
                <div class="row">
                    <div class="col-md-12 animate-box" data-animate-effect="fadeInLeft">
                        <p>
                            "A practical mix of languages, frameworks, and tools I reach for most often. "
                            "Always learning more — currently going deep on Rust + distributed systems."
                        </p>
                    </div>
                    <SkillBar name="Rust" color="color-1" pct=85 effect="fadeInLeft" />
                    <SkillBar name="TypeScript" color="color-2" pct=90 effect="fadeInRight" />
                    <SkillBar name="Leptos / React" color="color-3" pct=80 effect="fadeInLeft" />
                    <SkillBar name="TailwindCSS" color="color-4" pct=90 effect="fadeInRight" />
                    <SkillBar name="Postgres / SQL" color="color-5" pct=75 effect="fadeInLeft" />
                    <SkillBar name="Docker / DevOps" color="color-6" pct=70 effect="fadeInRight" />
                </div>
            </div>
        </section>
    }
}

#[component]
fn SkillBar(
    name: &'static str,
    color: &'static str,
    pct: u32,
    effect: &'static str,
) -> impl IntoView {
    let bar_class = format!("progress-bar {}", color);
    let style = format!("width:{}%", pct);
    let pct_text = format!("{}%", pct);
    view! {
        <div class="col-md-6 animate-box" data-animate-effect=effect>
            <div class="progress-wrap">
                <h3>{name}</h3>
                <div class="progress">
                    <div
                        class=bar_class
                        role="progressbar"
                        aria-valuenow=pct.to_string()
                        aria-valuemin="0"
                        aria-valuemax="100"
                        style=style
                    >
                        <span>{pct_text}</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Education() -> impl IntoView {
    view! {
        <section class="colorlib-education" data-section="education">
            <div class="colorlib-narrow-content">
                <div class="row">
                    <div
                        class="col-md-6 col-md-offset-3 col-md-pull-3 animate-box"
                        data-animate-effect="fadeInLeft"
                    >
                        <span class="heading-meta">"Education"</span>
                        <h2 class="colorlib-heading animate-box">"Education"</h2>
                    </div>
                </div>
                <div class="row">
                    <div class="col-md-12 animate-box" data-animate-effect="fadeInLeft">
                        <div class="fancy-collapse-panel">
                            <div
                                class="panel-group"
                                id="accordion"
                                role="tablist"
                                aria-multiselectable="true"
                            >
                                <EducationPanel
                                    id="One"
                                    title="B.Sc. in Computer Science & Engineering"
                                    body="Studied algorithms, systems, databases, and software engineering fundamentals. Worked on academic + side projects in parallel."
                                    expanded=true
                                />
                                <EducationPanel
                                    id="Two"
                                    title="Higher Secondary (Science)"
                                    body="Math, physics, chemistry, biology — the broad base that made later self-teaching easier."
                                    expanded=false
                                />
                                <EducationPanel
                                    id="Three"
                                    title="Self-taught Web Development"
                                    body="Started with HTML/CSS/jQuery, moved through React + Node.js, and finally arrived at Rust + Leptos for full-stack."
                                    expanded=false
                                />
                                <EducationPanel
                                    id="Four"
                                    title="Online Coursework (ongoing)"
                                    body="Continually picking up new things — distributed systems, database internals, design patterns, security basics."
                                    expanded=false
                                />
                                <EducationPanel
                                    id="Five"
                                    title="Secondary School Certificate"
                                    body="The very first chapter — where curiosity about computers turned into something to chase seriously."
                                    expanded=false
                                />
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn EducationPanel(
    id: &'static str,
    title: &'static str,
    body: &'static str,
    expanded: bool,
) -> impl IntoView {
    let heading_id = format!("heading{}", id);
    let collapse_id = format!("collapse{}", id);
    let collapse_href = format!("#collapse{}", id);
    let collapse_class = if expanded {
        "panel-collapse collapse in"
    } else {
        "panel-collapse collapse"
    };
    let anchor_class = if expanded { "" } else { "collapsed" };

    view! {
        <div class="panel panel-default">
            <div class="panel-heading" role="tab" id=heading_id.clone()>
                <h4 class="panel-title">
                    <a
                        class=anchor_class
                        data-toggle="collapse"
                        data-parent="#accordion"
                        href=collapse_href
                        aria-expanded=if expanded { "true" } else { "false" }
                        aria-controls=collapse_id.clone()
                    >
                        {title}
                    </a>
                </h4>
            </div>
            <div
                id=collapse_id
                class=collapse_class
                role="tabpanel"
                aria-labelledby=heading_id
            >
                <div class="panel-body">
                    <p>{body}</p>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Experience() -> impl IntoView {
    view! {
        <section class="colorlib-experience" data-section="experience">
            <div class="colorlib-narrow-content">
                <div class="row">
                    <div
                        class="col-md-6 col-md-offset-3 col-md-pull-3 animate-box"
                        data-animate-effect="fadeInLeft"
                    >
                        <span class="heading-meta">"Experience"</span>
                        <h2 class="colorlib-heading animate-box">"Work Experience"</h2>
                    </div>
                </div>
                <div class="row">
                    <div class="col-md-12">
                        <div class="timeline-centered">
                            <TimelineEntry
                                color="color-1"
                                role="Full Stack Developer (Freelance)"
                                period="2024 — Present"
                                desc="Building Rust + TypeScript products end-to-end. Focus on clean architecture, fast iteration, and shipping things that work."
                                effect="fadeInLeft"
                            />
                            <TimelineEntry
                                color="color-2"
                                role="Frontend Developer"
                                period="2022 — 2024"
                                desc="Designed and shipped React + Svelte interfaces. Worked with TypeScript, Vite, Tailwind, and the modern frontend toolchain."
                                effect="fadeInRight"
                            />
                            <TimelineEntry
                                color="color-3"
                                role="Backend Engineer"
                                period="2021 — 2022"
                                desc="Node.js APIs, Postgres tuning, basic Docker + CI/CD. Learned the value of strong types the hard way."
                                effect="fadeInLeft"
                            />
                            <TimelineEntry
                                color="color-4"
                                role="Web Developer (Junior)"
                                period="2020 — 2021"
                                desc="HTML, CSS, jQuery — building responsive sites and learning what a maintainable codebase actually looks like."
                                effect="fadeInTop"
                            />
                            <TimelineEntry
                                color="color-5"
                                role="Self-driven Learner"
                                period="2018 — 2020"
                                desc="The slow climb — first lines of code, first deploys, first time fixing my own bug at 2am."
                                effect="fadeInLeft"
                            />
                            <article class="timeline-entry begin animate-box" data-animate-effect="fadeInBottom">
                                <div class="timeline-entry-inner">
                                    <div class="timeline-icon color-none"></div>
                                </div>
                            </article>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn TimelineEntry(
    color: &'static str,
    role: &'static str,
    period: &'static str,
    desc: &'static str,
    effect: &'static str,
) -> impl IntoView {
    let icon_class = format!("timeline-icon {}", color);
    view! {
        <article class="timeline-entry animate-box" data-animate-effect=effect>
            <div class="timeline-entry-inner">
                <div class=icon_class>
                    <i class="icon-pen2"></i>
                </div>
                <div class="timeline-label">
                    <h2>
                        <a href="#">{role}</a>
                        " "
                        <span>{period}</span>
                    </h2>
                    <p>{desc}</p>
                </div>
            </div>
        </article>
    }
}

#[component]
fn Work() -> impl IntoView {
    view! {
        <section class="colorlib-work" data-section="work">
            <div class="colorlib-narrow-content">
                <div class="row">
                    <div
                        class="col-md-6 col-md-offset-3 col-md-pull-3 animate-box"
                        data-animate-effect="fadeInLeft"
                    >
                        <span class="heading-meta">"My Work"</span>
                        <h2 class="colorlib-heading animate-box">"Recent Work"</h2>
                    </div>
                </div>
                <div
                    class="row row-bottom-padded-sm animate-box"
                    data-animate-effect="fadeInLeft"
                >
                    <div class="col-md-12">
                        <p class="work-menu">
                            <span><a href="#" class="active">"All"</a></span>" "
                            <span><a href="#">"Web"</a></span>" "
                            <span><a href="#">"Backend"</a></span>" "
                            <span><a href="#">"CLI"</a></span>
                        </p>
                    </div>
                </div>
                <div class="row">
                    <WorkCard
                        img="/images/img-1.jpg"
                        title="Portfolio (Rust + Leptos)"
                        kind="Website"
                        effect="fadeInLeft"
                    />
                    <WorkCard
                        img="/images/img-2.jpg"
                        title="Task Tracker"
                        kind="Web App"
                        effect="fadeInRight"
                    />
                    <WorkCard
                        img="/images/img-3.jpg"
                        title="Weather CLI"
                        kind="Terminal Tool"
                        effect="fadeInTop"
                    />
                    <WorkCard
                        img="/images/img-4.jpg"
                        title="Image Optimizer"
                        kind="Service"
                        effect="fadeInBottom"
                    />
                    <WorkCard
                        img="/images/img-5.jpg"
                        title="Markdown Presenter"
                        kind="Web App"
                        effect="fadeInLeft"
                    />
                    <WorkCard
                        img="/images/img-6.jpg"
                        title="URL Shortener"
                        kind="Service"
                        effect="fadeInRight"
                    />
                </div>
                <div class="row">
                    <div class="col-md-12 animate-box">
                        <p>
                            <a href="#" class="btn btn-primary btn-lg btn-load-more">
                                "Load more " <i class="icon-reload"></i>
                            </a>
                        </p>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn WorkCard(
    img: &'static str,
    title: &'static str,
    kind: &'static str,
    effect: &'static str,
) -> impl IntoView {
    let bg = format!("background-image: url({});", img);
    view! {
        <div class="col-md-6 animate-box" data-animate-effect=effect>
            <div class="project" style=bg>
                <div class="desc">
                    <div class="con">
                        <h3><a href="#">{title}</a></h3>
                        <span>{kind}</span>
                        <p class="icon">
                            <span><a href="#"><i class="icon-share3"></i></a></span>
                            <span><a href="#"><i class="icon-eye"></i>" 100"</a></span>
                            <span><a href="#"><i class="icon-heart"></i>" 49"</a></span>
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Blog() -> impl IntoView {
    view! {
        <section class="colorlib-blog" data-section="blog">
            <div class="colorlib-narrow-content">
                <div class="row">
                    <div
                        class="col-md-6 col-md-offset-3 col-md-pull-3 animate-box"
                        data-animate-effect="fadeInLeft"
                    >
                        <span class="heading-meta">"Read"</span>
                        <h2 class="colorlib-heading">"Recent Blog"</h2>
                    </div>
                </div>
                <div class="row">
                    <BlogEntry
                        img="/images/blog-1.jpg"
                        date="May 15, 2026"
                        category="Rust"
                        comments="4"
                        title="Why I chose Rust + Leptos for my portfolio"
                        body="A pragmatic look at the tradeoffs of using a young framework for a personal site."
                        effect="fadeInLeft"
                    />
                    <BlogEntry
                        img="/images/blog-2.jpg"
                        date="May 10, 2026"
                        category="Architecture"
                        comments="2"
                        title="Clean architecture in Rust with a Cargo workspace"
                        body="Splitting a project into domain / application / infrastructure / presentation crates."
                        effect="fadeInRight"
                    />
                    <BlogEntry
                        img="/images/blog-3.jpg"
                        date="April 28, 2026"
                        category="Frontend"
                        comments="6"
                        title="Three small TailwindCSS v4 tips"
                        body="What changed in v4 and the conventions I now follow."
                        effect="fadeInLeft"
                    />
                </div>
                <div class="row">
                    <div class="col-md-12 animate-box">
                        <p>
                            <a href="#" class="btn btn-primary btn-lg btn-load-more">
                                "Load more " <i class="icon-reload"></i>
                            </a>
                        </p>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn BlogEntry(
    img: &'static str,
    date: &'static str,
    category: &'static str,
    comments: &'static str,
    title: &'static str,
    body: &'static str,
    effect: &'static str,
) -> impl IntoView {
    view! {
        <div class="col-md-4 col-sm-6 animate-box" data-animate-effect=effect>
            <div class="blog-entry">
                <a href="#" class="blog-img">
                    <img src=img class="img-responsive" alt=title />
                </a>
                <div class="desc">
                    <span>
                        <small>{date}</small>" | "
                        <small>" "{category}" "</small>" | "
                        <small><i class="icon-bubble3"></i>" "{comments}</small>
                    </span>
                    <h3><a href="#">{title}</a></h3>
                    <p>{body}</p>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Contact() -> impl IntoView {
    view! {
        <section class="colorlib-contact" data-section="contact">
            <div class="colorlib-narrow-content">
                <div class="row">
                    <div
                        class="col-md-6 col-md-offset-3 col-md-pull-3 animate-box"
                        data-animate-effect="fadeInLeft"
                    >
                        <span class="heading-meta">"Get in Touch"</span>
                        <h2 class="colorlib-heading">"Contact"</h2>
                    </div>
                </div>
                <div class="row">
                    <div class="col-md-5">
                        <div
                            class="colorlib-feature colorlib-feature-sm animate-box"
                            data-animate-effect="fadeInLeft"
                        >
                            <div class="colorlib-icon">
                                <i class="icon-globe-outline"></i>
                            </div>
                            <div class="colorlib-text">
                                <p><a href="mailto:mdazharcse14@gmail.com">"mdazharcse14@gmail.com"</a></p>
                            </div>
                        </div>

                        <div
                            class="colorlib-feature colorlib-feature-sm animate-box"
                            data-animate-effect="fadeInLeft"
                        >
                            <div class="colorlib-icon">
                                <i class="icon-map"></i>
                            </div>
                            <div class="colorlib-text">
                                <p>"Dhaka, Bangladesh"</p>
                            </div>
                        </div>

                        <div
                            class="colorlib-feature colorlib-feature-sm animate-box"
                            data-animate-effect="fadeInLeft"
                        >
                            <div class="colorlib-icon">
                                <i class="icon-phone"></i>
                            </div>
                            <div class="colorlib-text">
                                <p><a href="https://github.com/azharcse14">"github.com/azharcse14"</a></p>
                            </div>
                        </div>
                    </div>
                    <div class="col-md-7 col-md-push-1">
                        <div class="row">
                            <div
                                class="col-md-10 col-md-offset-1 col-md-pull-1 animate-box"
                                data-animate-effect="fadeInRight"
                            >
                                <form action="">
                                    <div class="form-group">
                                        <input
                                            type="text"
                                            class="form-control"
                                            placeholder="Name"
                                        />
                                    </div>
                                    <div class="form-group">
                                        <input
                                            type="text"
                                            class="form-control"
                                            placeholder="Email"
                                        />
                                    </div>
                                    <div class="form-group">
                                        <input
                                            type="text"
                                            class="form-control"
                                            placeholder="Subject"
                                        />
                                    </div>
                                    <div class="form-group">
                                        <textarea
                                            id="message"
                                            cols="30"
                                            rows="7"
                                            class="form-control"
                                            placeholder="Message"
                                        ></textarea>
                                    </div>
                                    <div class="form-group">
                                        <input
                                            type="submit"
                                            class="btn btn-primary btn-send-message"
                                            value="Send Message"
                                        />
                                    </div>
                                </form>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
