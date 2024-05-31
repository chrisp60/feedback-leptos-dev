use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-dev.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main style="background-color: lightblue; padding: 10px">
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let files = ["a.txt", "b.txt", "c.txt"];
    let labels = files.iter().copied().map(Into::into).collect();
    let tabs = move || {
        files
            .into_iter()
            .enumerate()
            .map(|(index, filename)| {
                let content = std::fs::read_to_string(filename).unwrap();
                view! {
                    <Tab index>
                        <div style="background-color: lightblue; padding: 10px">
                            <h2>{filename.to_string()}</h2>
                            <p>{content}</p>
                        </div>
                    </Tab>
                }
            })
            .collect_view()
    };

    view! {
        <h1>"Welcome to Leptos!"</h1>

        <Counter/>

        <p>"Click any of the tabs below to read a recipe."</p>
        <Tabs labels>
            <div>{tabs()}</div>
        </Tabs>
    }
}

#[island]
fn Counter() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

#[island]
fn Tab(index: usize, children: Children) -> impl IntoView {
    let selected = expect_context::<ReadSignal<usize>>();
    view! {
        <div
            style:background-color="lightgreen"
            style:padding="10px"
            style:display=move || if selected() == index {
                "block"
            } else {
                "none"
            }
        >
            {children()}
        </div>
    }
}

#[island]
fn Tabs(labels: Vec<String>, children: Children) -> impl IntoView {
    let (selected, set_selected) = create_signal(0);
    provide_context(selected);

    let buttons = labels
        .into_iter()
        .enumerate()
        .map(|(index, label)| {
            view! {
                <button on:click=move |_| set_selected(index)>
                    {label}
                </button>
            }
        })
        .collect_view();
    view! {
        <div
            style="display: flex; width: 100%; justify-content: space-around;\
            background-color: lightgreen; padding: 10px;"
        >
            {buttons}
        </div>
        {children()}
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
