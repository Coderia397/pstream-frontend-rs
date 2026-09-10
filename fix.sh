sed -i 's/icon: view! { \(.*\) }; AnyView::new(icon),/icon: AnyView::new(view! { \1 }),/g' src/components/layout/navbar.rs
