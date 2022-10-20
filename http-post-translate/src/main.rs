use serde_json::Value;
use reqwest::header;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configurando los headers necesarios
    let mut headers = header::HeaderMap::new();
    headers.insert("Accept-Encoding", header::HeaderValue::from_static("application/x-www-form-urlencoded"));
    // Reemplaza con tu propia key
    headers.insert("X-RapidAPI-Key", header::HeaderValue::from_static("<AGREGA ACA TU KEY>"));
    headers.insert("X-RapidAPI-Host", header::HeaderValue::from_static("text-translator2.p.rapidapi.com"));

    // Creando el body para el envio de un FORM
    let params = [
        ("source_language", "en"),
        ("target_language", "es"),
        ("text", "Hello World!")
    ];

    // Una buena idea si se van hacer muchos requests
    // es crear un "Cliente" el cual se puede reutilizar
    // para varias peticiones, ahora tambien agregamos los headers
    let cliente = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    // Ahora las peticiones las podemos hacer con el ciente
    // en lugar de utilizar directamente el crate
    let url = "https://text-translator2.p.rapidapi.com/translate";

    let r = cliente.post(url)
            .form(&params)
            .send()
            .await?;
    
    // Mostremos el codigo de respuesta enviado por el servicio
    println!("Este es el status code retornado por el servicio");
    println!("{}", r.status().as_u16());

    let resp_json = r.json::<Value>().await?;

    println!("Respuesta en formato JSON");
    println!("{:?}", resp_json);

    Ok(())
}