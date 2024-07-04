#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use leptos_dev::app::{
        state::{AppState, DatabaseConnection},
        *,
    };

    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let _db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    // mock connection to database
    let conn = DatabaseConnection;

    // get configurations
    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;

    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);
    // (chrisp60): already have a tracing-subscriber setup.
    // we can use traces instead of printlns. Logs will look a lot better
    // and can be more flexible with use of tracing's configurations
    //
    // Docs: https://docs.rs/tracing/latest/tracing/#using-the-macros
    // - % = use the implementation of std::fmt::Display to record this value.
    // - Logs will show as "server.addr = [address value]"
    // - For simple things, println is fine. But, there is an implicit cost to setting up a
    // suscriber so you might as well use it. Also, there is a possibly larger cost to using
    // println!, as each call to each needs to hold a lock on stdin. This can have implications for
    // code that is aiming to be highly performant in an async context.
    tracing::info!(server.addr = %addr);

    let state = AppState { conn };

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .app_data(web::Data::new(state.clone()))
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", site_root))
            // serve the favicon from /favicon.icon
            .service(favicon)
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
            .app_data(web::Data::new(leptos_options.to_owned()))
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use leptos_dev::app::*;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
