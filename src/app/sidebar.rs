use super::*;

/// Routes used in [`Sidebar`] definition.
const ROUTES: &[(&str, &str)] = &[
    ("/employees", "Employees"),
    ("/clients", "Clients"),
    ("/jobs", "Jobs"),
    ("/calendar", "Calendar"),
    ("/reports", "Reports"),
    ("/settings", "Settings"),
];

#[island]
pub fn Sidebar() -> impl IntoView {
    let nav_items = ROUTES
        .iter()
        .map(|(href, text)| {
            view! {
                <div>
                    <a href=*href class="std-btn">
                        {*text}
                    </a>
                </div>
            }
        })
        .collect_view();
    view! { <div class="sidebar text-center border-2 border-secondary-400 p-3 mt-3">{nav_items}</div> }
}
