#[cfg(feature = "poem")]
use poem::{endpoint::make_sync, web::Html, Endpoint};

use include_base64::include_base64;

const SWAGGER_UI_JS: &str = concat!(
    "<style charset=\"UTF-8\">\n",
    include_str!("swagger-ui.css"),
    "\n</style>"
);

const SWAGGER_UI_CSS: &str = concat!(
    "<script charset=\"UTF-8\">\n",
    include_str!("swagger-ui-bundle.js"),
    "\n</script>"
);

const SWAGGER_FAVICON_32: &str = concat!(
    r#"<link rel="icon" href="data;image/png;base64,"#,
    include_base64!("src/favicon-32x32.png"),
    r#"" sizes="32x32">"#
);

const SWAGGER_FAVICON_16: &str = concat!(
    r#"<link rel="icon" href="data;image/png;base64,"#,
    include_base64!("src/favicon-16x16.png"),
    r#"" sizes="16x16">"#
);

const OAUTH2_REDIRECT_HTML: &str = include_str!("oauth2-redirect.html");

//https://swagger.io/docs/open-source-tools/swagger-ui/usage/configuration/
const SWAGGER_UI_TEMPLATE: &str = include_str!("index.html");

#[derive(Debug, Default)]
pub struct Options<'a> {
    pub url: Option<&'a str>,
    pub script: Option<&'a str>,
    pub persist_authorization: Option<bool>,
}

fn create_html(options: Options) -> String {
    SWAGGER_UI_TEMPLATE
        .replace("{:style}", SWAGGER_UI_CSS)
        .replace("{:script}", SWAGGER_UI_JS)
        .replace("{:favicon32}", SWAGGER_FAVICON_32)
        .replace("{:favicon16}", SWAGGER_FAVICON_16)
        .replace(
            "$url$",
            &options
                .url
                .map(|x| format!("\"{}\"", x))
                .unwrap_or("null".to_string()),
        )
        .replace("$inject$", options.script.unwrap_or(""))
        .replace(
            "$persist_authorization$",
            (if options.persist_authorization.unwrap_or(false) {
                "true"
            } else {
                "false"
            }),
        )
}

/// Creates the HTML for Swagger UI.
///
/// This module provides functions to generate the HTML required to embed Swagger UI in a web application.
/// It includes the necessary CSS and JavaScript files.
/// You can get the oauth-receiver.html file as well with the `get_oauth_receiver_html` function.
///
/// # Example
///
/// ```rust
/// use swagger_ui_embed::{get_html, Options};
///
/// let options = Options {
///     url: Some("https://petstore.swagger.io/v2/swagger.json"),
///     script: None,
///     persist_authorization: Some(true),
/// };
///
/// let html = get_html(options);
/// println!("{}", html);
/// ```
///
/// This will generate the HTML for Swagger UI with the specified options.
pub fn get_html(options: Options) -> String {
    create_html(options)
}

pub fn get_oauth2_redirect_html() -> String {
    OAUTH2_REDIRECT_HTML.to_string()
}

#[cfg(feature = "poem")]
pub fn create_endpoint(options: Options) -> impl Endpoint {
    let ui_html = create_html(options);
    poem::Route::new()
        .at("/", make_sync(move |_| Html(ui_html.clone())))
        .at(
            "/oauth-receiver.html",
            make_sync(move |_| Html(OAUTH_REDIRECT_HTML.to_string())),
        )
}
