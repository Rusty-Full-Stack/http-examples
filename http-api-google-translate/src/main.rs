use serde_json::Value;
use reqwest::header;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configurando los headers necesarios
    let mut headers = header::HeaderMap::new();
    headers.insert("Accept-Encoding", header::HeaderValue::from_static("application/gzip"));
    // Reemplaza con tu propia key
    headers.insert("X-RapidAPI-Key", header::HeaderValue::from_static("<AGREGA ACA TU KEY>"));
    headers.insert("X-RapidAPI-Host", header::HeaderValue::from_static("google-translate1.p.rapidapi.com"));

    // Una buena idea si se van hacer muchos requests
    // es crear un "Cliente" el cual se puede reutilizar
    // para varias peticiones, ahora tambien agregamos los headers
    let cliente = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    // Ahora las peticiones las podemos hacer con el ciente
    // en lugar de utilizar directamente el crate
    let url = "https://google-translate1.p.rapidapi.com/language/translate/v2/languages";

    let r = cliente.get(url)
            .send()
            .await?;
    
    // Ahora mostremos en pantalla la composicion completa de la respuesta.
    println!("Esta es la respuesta en crudo");
    println!("{:?}", r);

    // Mostremos el codigo de respuesta enviado por el servicio
    println!("Este es el status code retornado por el servicio");
    println!("{}", r.status().as_u16());

    let resp_json = r.json::<Value>().await?;

    println!("Respuesta mas clara en formato JSON");
    println!("{:?}", resp_json);

    Ok(())
}