use yew::{function_component, html, Html};

#[function_component(Terminal)]
pub fn terminal() -> Html {
    html! {
        <div class="w-1/2 mx-auto">
            <div class="w-full shadow-2xl subpixel-antialiased rounded h-64 bg-black border-black mx-auto">
                <div
                    id="terminal-header"
                    class="flex items-center h-6 rounded-t bg-gray-100 border-b border-gray-500 text-center text-black"
                >
                    <div
                        id="closebtn"
                        class="flex ml-2 items-center text-center border-red-900 bg-red-500 shadow-inner rounded-full w-3 h-3"
                    >
                    </div>
                    <div
                        id="minbtn"
                        class="ml-2 border-yellow-900 bg-yellow-500 shadow-inner rounded-full w-3 h-3"
                    >
                    </div>
                    <div
                        id="maxbtn"
                        class="ml-2 border-green-900 bg-green-500 shadow-inner rounded-full w-3 h-3"
                    >
                    </div>
                    <div id="terminal-title" class="mx-auto pr-16">
                        <p class="text-center text-sm">{"Terminal"}</p>
                    </div>
                </div>
                <div id="console" class="pl-1 pt-1 h-auto text-green-200 font-mono text-xs bg-black">
                    <p class="pb-1">{"Last login: Wed Sep 25 09:11:04 on ttys002"}</p>
                    <p class="pb-1">{"geoffjay:projects geoffjay$"}</p>
                </div>
            </div>
        </div>
    }
}
