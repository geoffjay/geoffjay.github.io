use yew::{classes, function_component, html, Children, Html, Properties};
use yew_tailwind::components::Container;

#[derive(PartialEq, Properties)]
pub struct TimelineProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Timeline)]
pub fn timeline(props: &TimelineProps) -> Html {
    let classes = vec![
        "flex",
        "flex-col",
        "md:grid",
        "grid-cols-9",
        "mx-auto",
        "p-2",
        "text-gray-800",
    ];

    html! {
        <div class={classes!(classes)}>
            {props.children.clone()}
        </div>
    }
}

#[function_component(TimelineItemBullet)]
pub fn timeline_item_bullet() -> Html {
    html! {
        <div class="col-start-5 col-end-6 md:mx-auto relative mr-10">
            <div class="h-full w-6 flex items-center justify-center">
                <div class="h-full w-0.5 bg-gray-500 pointer-events-none"></div>
            </div>
            <div class="w-6 h-6 absolute top-1/2 -mt-3 rounded-full border-2 border-gray-500 bg-gray-200 shadow"></div>
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TimelineItemPosition {
    Left,
    Right,
}

#[derive(PartialEq, Properties)]
pub struct TimelineItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub title: String,
    #[prop_or(TimelineItemPosition::Left)]
    pub position: TimelineItemPosition,
}

#[function_component(TimelineItem)]
pub fn timeline_item(props: &TimelineItemProps) -> Html {
    let TimelineItemProps {
        children,
        title,
        position,
    } = props;
    let item_classes = vec![
        "flex",
        "md:contents",
        match position {
            TimelineItemPosition::Left => "flex-row-reverse",
            TimelineItemPosition::Right => "",
        },
    ];

    html! {
        <div class={classes!(item_classes)}>
            {match position {
                TimelineItemPosition::Left => html! {
                    <>
                        <div class="bg-gray-200 border-2 border-gray-500 hover:border-red col-start-1 col-end-5 p-4 rounded-lg my-4 ml-auto shadow-md">
                            <h3 class="font-semibold text-lg mb-1">{title.clone()}</h3>
                            <p class="leading-tight text-justify">
                                {children.clone()}
                            </p>
                        </div>
                        <TimelineItemBullet />
                    </>

                },
                TimelineItemPosition::Right => html! {
                    <>
                        <TimelineItemBullet />
                        <div class="bg-gray-200 border-2 border-gray-500 hover:border-2 hover:border-red col-start-6 col-end-10 p-4 rounded-xl my-4 mr-auto shadow-md">
                            <h3 class="font-semibold text-lg mb-1">{"Lorem ipsum"}</h3>
                            <p class="leading-tight text-justify">
                                {children.clone()}
                            </p>
                        </div>
                    </>
                },
            }}
        </div>
    }
}

#[function_component(Resume)]
pub fn resume() -> Html {
    html! {
        <Container classes={classes!("w-full", "mx-auto")}>
            <div class="text-xl pb-6">{"Resume"}</div>
            <Timeline>
                <TimelineItem title="Lorem ipsum" position={TimelineItemPosition::Left}>
                    {"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi, quaerat?"}
                </TimelineItem>
                <TimelineItem title="Lorem ipsum" position={TimelineItemPosition::Right}>
                    {"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi, quaerat?"}
                </TimelineItem>
                <TimelineItem title="Lorem ipsum" position={TimelineItemPosition::Left}>
                    {"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi, quaerat?"}
                </TimelineItem>
                <TimelineItem title="Lorem ipsum" position={TimelineItemPosition::Left}>
                    {"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi, quaerat?"}
                </TimelineItem>
                <TimelineItem title="Lorem ipsum" position={TimelineItemPosition::Right}>
                    {"Lorem ipsum dolor sit amet consectetur adipisicing elit. Modi, quaerat?"}
                </TimelineItem>
            </Timeline>
        </Container>
    }
}
