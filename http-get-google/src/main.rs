#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let html: String = reqwest::get("https://www.google.com") // Se pasa el url o endpoint con el cual se interacturara
                        .await?
                        .text() //Convierte el resultado en un String
                        .await?;
    println!("{:?}", html);

    Ok(())
}