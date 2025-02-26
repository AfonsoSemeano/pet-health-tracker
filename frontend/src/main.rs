use core::panic;

use leptos::prelude::*;
use leptos::view;
use leptos_router::components::{Router, Routes, Route};
use leptos_router::path;
use reqwasm::http::Request;
use serde::Deserialize;


fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(|| view! { <App /> })
}

#[component]
fn App() -> impl IntoView {


    view! {

        <div>This is a test</div>
        <Router>
            <nav>
              /* ... */
            </nav>
            <main>
              <Routes fallback=|| "Not found.">
                <Route path=path!("/") view=Buttons/>
              </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Buttons() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <FetchPets />

    }
}

#[derive(Deserialize, Clone)]
struct Pet {
    id: u32,
    name: String,
    size: String,
    birthday: String,
    weight: u32,
}

#[derive(Deserialize)]
struct PetApiResponse {
    data: Vec<Pet>,
}



#[component]
fn FetchPets() -> impl IntoView {
    // Create a signal to store the fetched posts
    let (pets, set_pets) = signal(Vec::<Pet>::new());
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);

    // Use an effect to fetch data once the component mounts
    let async_data = LocalResource::new(move || {
        load_pets(loading.get())
    });

    /*let async_result = move || -> Vec<_> {
        async_data
            .get()
            .as_deref()
            .map(|value| view! {
                {value.iter().map(|pet| view! {<li>{pet.name.clone()}</li>}).collect_view()}
            })
            .unwrap_or_else(move || {
                Vec::new()
            })
    };*/

    async fn load_pets(is_loading: bool) -> Result<Vec<String>> {
            let result = Request::get("http://localhost:3000/")
            .send()
            .await?
            .json::<Vec<Pet>>()
            .await?
            .into_iter()
            .map(|pet| pet.name)
            .collect::<Vec<_>>();
            
            return Ok(result);
    }


    // Render the posts or show a loading message or an error
    view! {
        <div>
            /*{if loading.get() {
                view! { <p>{"Loading...".to_string()}</p> }
            } else if let Some(err) = error.get() {
                view! { <p>{err.to_string()}</p> }
            } else {
                view! {
                    <p>{"stuff".to_string()}</p>
                }
            }}*/
            
                <ul>
                    {move || Suspend::new(async move {
                        async_data.await
                            .map(|pets| {
                                pets.iter()
                                    .map(|p| {
                                        view! {
                                            <li>
                                                {p.clone()}
                                            </li>
                                        }
                                    }).collect::<Vec<_>>()
                            })
                    })}
                </ul>
        </div>
    }
}