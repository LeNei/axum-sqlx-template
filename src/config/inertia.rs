use axum_inertia::{InertiaConfig, vite};

pub fn get_inertia_config(is_dev: bool) -> InertiaConfig {
    if !is_dev {
        vite::Production::new("frontend/dist/.vite/manifest.json", "src/main.tsx")
            .unwrap()
            .lang("de")
            .into_config()
    } else {
        vite::Development::default()
            .port(5173)
            .main("src/main.tsx")
            .lang("de")
            .react() // call if using react
            .into_config()
    }
}
