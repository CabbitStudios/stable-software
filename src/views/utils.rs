use maud::{Markup, html};

pub fn button_group(buttons: Vec<Markup>) -> Markup {
    html! {
        .field.is-grouped {
            @for btn in buttons {
                .control {
                    (btn)
                }
            }
        }
    }
}