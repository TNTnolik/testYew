use yew::{function_component, html, Html};

#[function_component]
pub fn Header() -> Html {
    html! {
        <header>
            <div class="menu">
                <h1>
                    {"Logo"}
                </h1>
                <nav>
                    <ul>
                        <li>
                            {"test"}
                        </li>
                        <li>
                            {"test 2"}
                        </li>
                        <li>
                            {"test 3"}
                        </li>
                    </ul>
                </nav>
            </div>
        </header>
    }
}
