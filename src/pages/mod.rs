use yew::prelude::*;

mod home_page;

use crate::router::routes::*;

pub fn switch(routes: &Route) -> Html {
  match routes {
      Route::Home => html! { <home_page::HomePage /> },
      Route::NotFound => html! { <h1>{ "404" }</h1> },
  }
}
