use poem::{endpoint::make_sync, web::Html, Endpoint};

const SWAGGER_UI_JS: &str = concat!(
    "<style charset=\"UTF-8\">",
    include_str!("swagger-ui-bundle.js"),
    "</style>"
);

const SWAGGER_UI_CSS: &str = concat!(
    "script charset=\"UTF-8\">",
    include_str!("swagger-ui.css"),
    "</script>"
);

const OAUTH_RECEIVER_HTML: &str = include_str!("oauth-receiver.html");

//https://swagger.io/docs/open-source-tools/swagger-ui/usage/configuration/
const SWAGGER_UI_TEMPLATE: &str = include_str!("index.html");

#[derive(Debug, Default)]
pub struct Options<'a> {
    pub url: Option<&'a str>,
    pub script: Option<&'a str>,
}

fn create_html(options: Options) -> String {
    SWAGGER_UI_TEMPLATE
        .replace("{:style}", SWAGGER_UI_CSS)
        .replace("{:script}", SWAGGER_UI_JS)
        .replace("$url$", options.url.unwrap_or("null"))
        .replace(
            "{:inject}",
            &options
                .script
                .map(|script| format!("{}{}{}", "<script>", script, "</script>"))
                .unwrap_or("".to_owned()),
        )
}

pub fn create_endpoint(options: Options) -> impl Endpoint {
    let ui_html = create_html(options);
    poem::Route::new()
        .at("/", make_sync(move |_| Html(ui_html.clone())))
        .at(
            "/oauth-receiver.html",
            make_sync(move |_| Html(OAUTH_RECEIVER_HTML.to_string())),
        )
}
