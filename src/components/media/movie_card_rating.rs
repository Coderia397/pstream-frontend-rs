use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MovieRating {
    Like,
    Dislike,
    Love,
}

#[component]
pub fn DoubleThumbsUpIcon(
    #[prop(default = 22)] size: u32,
    #[prop(default = "#2f2f2f".to_string())] mask_color: String,
) -> impl IntoView {
    let offset_x = (size as f64 * 0.38).round() as i32;
    let offset_y = (size as f64 * 0.32).round() as i32;
    let bite_r = (size as f64 * 0.44).round() as i32;

    let width = size as i32 + offset_x;
    let height = size as i32 + offset_y;

    let mask_style = format!(
        "width: {}px; height: {}px; left: {}px; top: {}px; background: {}; z-index: 2;",
        bite_r * 2, bite_r * 2, offset_x - bite_r + 4, offset_y - bite_r + 5, mask_color
    );

    view! {
        <div class="relative inline-flex" style=format!("width: {}px; height: {}px;", width, height)>
            <div class="absolute" style=format!("left: {}px; top: {}px; z-index: 1;", offset_x, offset_y)>
                <i class="ph-bold ph-thumbs-up" style=format!("font-size: {}px;", size)></i>
            </div>
            <div class="absolute rounded-full" style=mask_style></div>
            <div class="absolute" style="left: 0; top: 0; z-index: 3;">
                <i class="ph-bold ph-thumbs-up" style=format!("font-size: {}px;", size)></i>
            </div>
        </div>
    }
}

#[component]
pub fn RatingIcon(
    rating: Option<MovieRating>,
    #[prop(default = 22)] size: u32,
    #[prop(default = "#2f2f2f".to_string())] mask_color: String,
) -> impl IntoView {
    view! {
        {match rating {
            Some(MovieRating::Love) => view! {
                <DoubleThumbsUpIcon size=size mask_color=mask_color />
            }.into_any(),
            Some(MovieRating::Dislike) => view! {
                <i class="ph-bold ph-thumbs-down" style=format!("font-size: {}px;", size)></i>
            }.into_any(),
            _ => view! {
                <i class="ph-bold ph-thumbs-up" style=format!("font-size: {}px;", size)></i>
            }.into_any(),
        }}
    }
}
