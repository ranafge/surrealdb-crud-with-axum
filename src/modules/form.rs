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
            <h2>District Form</h2>
            <form action="/show_form" method="post">
                <label for="name">Name:</label><br>
                <input type="text" id="name" name="name"><br>
                
                <label for="population">Population:</label><br>
                <input type="number" id="population" name="population"><br>
                
                <label for="climate">Climate:</label><br>
                <input type="text" id="climate" name="climate"><br>
                
                <label for="international_border">International Border:</label><br>
                <input type="checkbox" id="international_border" name="international_border"><br>
                
                <label for="country">Country:</label><br>
                <input type="text" id="country" name="country"><br>
                
                <label for="immediate_upper_division">Immediate Upper Division:</label><br>
                <input type="text" id="immediate_upper_division" name="immediate_upper_division"><br><br>
                
                <input type="submit" value="Submit">
            </form>
            </body>
        </html>
        "#
    )
}