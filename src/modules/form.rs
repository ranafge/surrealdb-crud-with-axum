use axum::{response::Html, Form};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Input {
   pub name: String,
   pub email:String
}

pub async fn accept_from(Form(input): Form<Input>) {
    dbg!(&input);
}
pub async fn show_from() -> Html<&'static str> {
    Html(
        r#"
        <html>
            <head>
                <title>SurrealDB</title>
            </head>
            <body>
                <form method="post" action="/show_form">
                    <label for="name">Enter your name</label>
                    <input type="text" name="name" id="name">
                    <label for="email">Enter your email</label>
                    <input type="text" name="email" id="email">
                    <input type="submit" value="Subscribe!">
                </form>
                <h2>Thanks for submission</h2>
            </body>
        </html>
        "#
    )
}