use yew::prelude::*;
use yew_router::prelude::*;

mod router {
  pub mod routes;
}
mod pages;

use crate::router::routes::*;

#[function_component(App)]
fn app() -> Html {
  html! {
    <BrowserRouter>
      <Switch<Route> render={Switch::render(pages::switch)} />
    </BrowserRouter>
  }
}

fn main() {
  yew::start_app::<App>();
}
