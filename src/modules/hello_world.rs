

pub async fn hello() -> String {
    let name = "Ayesha";
    tracing::debug!("Hello handler successfully returned: {}", name);
    format!("Hello {}! How are you? ", name)
}