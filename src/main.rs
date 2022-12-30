#![allow(unused_parens)]
#![allow(clippy::needless_return)]

use pix_qrcode_gerencianet::{
    gerencianet::{get_token, Credentials, do_cobranca, Client, generate_qr_code, Configuration},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Load initial configuration (read from yaml file)
    let configuration = Configuration::new().unwrap();

    // Create credentials
    let credentials = Credentials::new(&configuration).unwrap();

    // Create a client instance and perform the authentication request
    let client = Client::new(&credentials, None).unwrap();
    let access_token = get_token(&client, &configuration).await.unwrap();

    // Create a new client with the `access_token` as default header
    let authenticated_client = Client::new(&credentials, Some(access_token)).unwrap();

    // Create a `cobranca`
    let cobranca = do_cobranca(&authenticated_client, &configuration).await.unwrap();

    // Get the QR code base64
    let location = generate_qr_code(&authenticated_client, &configuration, cobranca.loc.id).await.unwrap();

    dbg!(&location);
    // Save it as .png
    save_qr_code_image(location.imagem_qrcode).unwrap();

    return Ok(());
}


fn save_qr_code_image(base64: String) -> Result<(), anyhow::Error> {
    let base64 = base64.replace("data:image/png;base64,", "");
    let image_buffer = base64::decode(base64)
        .map_err(|e| anyhow::anyhow!(format!("Failed to decode image base64 string: {}", e)))?;

    let image = image::load_from_memory_with_format(&image_buffer, image::ImageFormat::Png)
        .map_err(|e| anyhow::anyhow!(format!("Failed to load qr code image: {}", e)))?;

    image.save("qrcode.png")
        .map_err(|e| anyhow::anyhow!(format!("Failed to save qr code image: {}", e)))?;
    return Ok(());
}