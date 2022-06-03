use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="container">
            <div class="notification is-primary">
                <h1 class="title">{"Yew Parcel Template"}</h1>
            </div>
            <div class="notification is-secondary">
                <h2 class="subtitle">{"Libraries used in this template"}</h2>
            </div>
            <section class="section">
                <ul>
                    <li>
                        <a href="https://yew.rs" target="_blank">
                            {"yew.rs"}
                        </a>
                        {" : rustwasm frontent framwork"}
                    </li>
                    <li>
                        <a href="https://github.com/spielrs/yew_styles" target="_blank">
                            {"yew_styles"}
                        </a>
                        {" : styles framework for yew"}
                    </li>
                    <li>
                        <a href="https://parceljs.org/" target="_blank">
                            {"parceljs"}
                        </a>
                        {" : builder js library"}
                    </li>
                    <li>
                        <a href="https://github.com/paulmillr/chokidar" target="_blank">
                            {"chokidar"}
                        </a>
                        {" : watcher js library"}
                    </li>
                </ul>
            </section>
        </div>
    }
}
