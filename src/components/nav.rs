use yew::{classes, function_component, html, use_state_eq, Callback, Children, Html, Properties};
use yew_router::prelude::*;

use crate::app::Route;
use crate::components::avatar::Avatar;

fn active_link_classes(current: &Route, link: &Route) -> Vec<String> {
    if current.clone() == link.clone() {
        vec!["bg-gray-300".to_string(), "dark-mode:bg-gray-800".to_string()]
    } else {
        vec!["bg-transparent".to_string(), "dark-mode:bg-transparent".to_string()]
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct AnimationWrapperProps {
    #[prop_or(false)]
    pub xyz: bool,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(AnimationWrapper)]
fn animation_wrapper(props: &AnimationWrapperProps) -> Html {
    let AnimationWrapperProps {
        xyz,
        children,
    } = props.clone();

    let mut node = html! {
        <div>{ for children.iter() }</div>
    };

    if xyz {
        if let yew::virtual_dom::VNode::VTag(tag) = &mut node {
            tag.add_attribute("xyz", "".to_string());
        }
    }

    node
}

#[derive(PartialEq, Properties, Clone)]
pub struct AnimationProps {
    pub xyz: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Animation)]
fn animation(props: &AnimationProps) -> Html {
    let AnimationProps {
        xyz,
        children,
    } = props.clone();

    let mut node = html! {
        <div class="xyz-in">{ for children.iter() }</div>
    };

    if let yew::virtual_dom::VNode::VTag(tag) = &mut node {
        tag.add_attribute("xyz", xyz.clone());
    }

    node
}

#[function_component(Nav)]
pub fn nav() -> Html {
    let current_route = use_route().unwrap();

    let navbar_active = use_state_eq(|| false);

    let toggle_navbar = {
        let navbar_active = navbar_active.clone();

        Callback::from(move |_| {
            navbar_active.set(!*navbar_active);
        })
    };

    let active_class = if !*navbar_active { "hidden" } else { "" };

    let icon_path = if *navbar_active {
        r#"
            M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414
            10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1
            1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z
        "#
        .to_string()
    } else {
        r#"
            M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1
            1 0 110 2H4a1 1 0 01-1-1zM9 15a1 1 0 011-1h6a1 1 0 110 2h-6a1 1 0 01-1-1z
        "#
        .to_string()
    };

    let avatar_url = "https://avatars.githubusercontent.com/u/206354?s=400&v=4".to_string();

    let container_classes = vec![
        "flex-col",
        "md:flex",
        "md:flex-row",
        "md:min-h-screen",
        "shadow-navRight",
        // "md:shadow-navBottom",
        "z-10",
    ];

    let link_classes = vec![
        "block",
        "px-8",
        "py-4",
        "text-sm",
        "font-semibold",
        "text-gray-800",
        "hover:text-gray-800",
        "hover:bg-gray-300",
        "focus:text-gray-800",
        "focus:bg-gray-300",
        "focus:outline-none",
        "focus:shadow-outline",
        "border-transparent",
        "border-r-4",
        "hover:border-r-4",
        "hover:border-red",
        "dark-mode:text-gray-300",
        "dark-mode:hover:bg-gray-500",
        "dark-mode:hover:text-white",
        "dark-mode:focus:bg-gray-500",
        "dark-mode:focus:text-white",
        "dark-mode:border-transparent",
        "dark-mode:border-r-4",
        "dark-mode:hover:border-r-4",
        "dark-mode:hover:border-red",
    ];

    html! {
        <div class={classes!(container_classes)}>
            <div class="flex flex-col w-full md:w-64 text-gray-700 bg-gray-200 dark-mode:text-gray-200 dark-mode:bg-gray-800 flex-shrink-0">
                <div class="flex-shrink-0 px-8 py-8 flex flex-row items-center justify-around">
                    <Link<Route>
                        classes={classes!(
                            "text-lg",
                            "font-semibold",
                            "tracking-widest",
                            "text-gray-800",
                            "uppercase",
                            "dark-mode:text-white",
                            "focus:outline-none",
                            "focus:shadow-outline",
                        )}
                        to={Route::Home}
                    >
                        <Avatar url={avatar_url} />
                    </Link<Route>>
                    <button
                        class="rounded-lg md:hidden rounded-lg focus:outline-none focus:shadow-outline"
                        onclick={toggle_navbar}
                    >
                        <svg fill="currentColor" viewBox="0 0 20 20" class="w-6 h-6">
                            <path fill-rule="evenodd" d={icon_path} clip-rule="evenodd"></path>
                        </svg>
                    </button>
                </div>
                <nav
                    class={classes!(
                        "flex-grow",
                        "md:block",
                        "md:pb-0",
                        "md:overflow-y-auto",
                        active_class,
                    )}
                >
                    <Animation xyz="fade left stagger">
                        <div class="xyz-nested">
                            <Link<Route>
                                classes={classes!(
                                    active_link_classes(&current_route, &Route::Blog),
                                    link_classes.clone(),
                                )}
                                to={Route::Blog}
                            >
                                { "Blog" }
                            </Link<Route>>
                        </div>
                        <div class="xyz-nested">
                            <Link<Route>
                                classes={classes!(
                                    active_link_classes(&current_route, &Route::Portfolio),
                                    link_classes.clone(),
                                )}
                                to={Route::Portfolio}
                            >
                                { "Portfolio" }
                            </Link<Route>>
                        </div>
                        <div class="xyz-nested">
                            <Link<Route>
                                classes={classes!(
                                    active_link_classes(&current_route, &Route::Resume),
                                    link_classes.clone(),
                                )}
                                to={Route::Resume}
                            >
                                { "Resume" }
                            </Link<Route>>
                        </div>
                        <div class="xyz-nested">
                            <Link<Route>
                                classes={classes!(
                                    active_link_classes(&current_route, &Route::About),
                                    link_classes.clone(),
                                )}
                                to={Route::About}
                            >
                                { "About" }
                            </Link<Route>>
                        </div>
                        <div class="xyz-nested">
                            <Link<Route>
                                classes={classes!(
                                    active_link_classes(&current_route, &Route::Contact),
                                    link_classes.clone(),
                                )}
                                to={Route::Contact}
                            >
                                { "Contact" }
                            </Link<Route>>
                        </div>
                    </Animation>
                    // <div @click.away="open = false" class="relative" x-data="{ open: false }">
                    //     <button @click="open = !open" class="flex flex-row items-center w-full px-4 py-2 mt-2 text-sm font-semibold text-left bg-transparent rounded-lg dark-mode:bg-transparent dark-mode:focus:text-white dark-mode:hover:text-white dark-mode:focus:bg-gray-600 dark-mode:hover:bg-gray-600 md:block hover:text-gray-900 focus:text-gray-900 hover:bg-gray-200 focus:bg-gray-200 focus:outline-none focus:shadow-outline">
                    //         <span>Dropdown</span>
                    //         <svg
                    //             fill="currentColor"
                    //             viewBox="0 0 20 20"
                    //             :class="{'rotate-180': open, 'rotate-0': !open}"
                    //             class="inline w-4 h-4 mt-1 ml-1 transition-transform duration-200 transform md:-mt-1"
                    //         >
                    //             <path
                    //                 fill-rule="evenodd"
                    //                 d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
                    //                 clip-rule="evenodd"
                    //             >
                    //             </path>
                    //         </svg>
                    //     </button>
                    //     <div x-show="open" x-transition:enter="transition ease-out duration-100" x-transition:enter-start="transform opacity-0 scale-95" x-transition:enter-end="transform opacity-100 scale-100" x-transition:leave="transition ease-in duration-75" x-transition:leave-start="transform opacity-100 scale-100" x-transition:leave-end="transform opacity-0 scale-95" class="absolute right-0 w-full mt-2 origin-top-right rounded-md shadow-lg">
                    //         <div class="px-2 py-2 bg-white rounded-md shadow dark-mode:bg-gray-800">
                    //             <a class="block px-4 py-2 mt-2 text-sm font-semibold bg-transparent rounded-lg dark-mode:bg-transparent dark-mode:hover:bg-gray-600 dark-mode:focus:bg-gray-600 dark-mode:focus:text-white dark-mode:hover:text-white dark-mode:text-gray-200 md:mt-0 hover:text-gray-900 focus:text-gray-900 hover:bg-gray-200 focus:bg-gray-200 focus:outline-none focus:shadow-outline" href="#">Link #1</a>
                    //             <a class="block px-4 py-2 mt-2 text-sm font-semibold bg-transparent rounded-lg dark-mode:bg-transparent dark-mode:hover:bg-gray-600 dark-mode:focus:bg-gray-600 dark-mode:focus:text-white dark-mode:hover:text-white dark-mode:text-gray-200 md:mt-0 hover:text-gray-900 focus:text-gray-900 hover:bg-gray-200 focus:bg-gray-200 focus:outline-none focus:shadow-outline" href="#">Link #2</a>
                    //             <a class="block px-4 py-2 mt-2 text-sm font-semibold bg-transparent rounded-lg dark-mode:bg-transparent dark-mode:hover:bg-gray-600 dark-mode:focus:bg-gray-600 dark-mode:focus:text-white dark-mode:hover:text-white dark-mode:text-gray-200 md:mt-0 hover:text-gray-900 focus:text-gray-900 hover:bg-gray-200 focus:bg-gray-200 focus:outline-none focus:shadow-outline" href="#">Link #3</a>
                    //         </div>
                    //     </div>
                    // </div>
                </nav>
            </div>
        </div>
    }
}
