use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct IconButtonProps {
    pub alt: String,
    pub icon: String,
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,
    pub ty: String,
}

#[component]
pub fn IconButton(props: IconButtonProps) -> Element {
    let onclick = props.onclick;

    rsx! {
        button {
            r#type: "{props.ty}",
            key: "{props.alt}",
            class: "flex flex-col items-center transition transform duration-300 hover:scale-110 hover:bg-gray-200 p-2 rounded",
            onclick: move |e| {
                if let Some(cb) = &onclick {
                    cb.call(e);
                }
            },
            img {
                class: "h-5 w-5 mb-1",
                src: props.icon,
                alt: "{props.alt}",
            }
        }
    }
}
