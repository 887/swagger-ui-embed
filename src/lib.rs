use poem::{endpoint::make_sync, web::Html, Endpoint};

const SWAGGER_UI_JS: &str = concat!(
    "<style charset=\"UTF-8\">\n",
    include_str!("swagger-ui-bundle.js"),
    "\n</style>"
);

const SWAGGER_UI_CSS: &str = concat!(
    "<script charset=\"UTF-8\">\n",
    include_str!("swagger-ui.css"),
    "\n</script>"
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
        .replace(
            "$url$",
            &options
                .url
                .map(|x| format!("\"{}\"", x))
                .unwrap_or("null".to_string()),
        )
        .replace("$inject$", options.script.unwrap_or(""))
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
