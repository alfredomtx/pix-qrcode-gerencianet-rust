# What is this project?
It's a simple working demonstration of generation of PIX QR Code from [Gerencianet](https://gerencianet.com.br/).
 
[Gerencianet](https://dev.gerencianet.com.br/docs) has amazing SDKs for many languages that makes it easier to make the integrations, but Rust is not one of them.
 
This app is supposed to be the **simplest** implementation of some of their endpoints for PIX QR Code generation, to be used as basis for anyone who is looking to integrate their Rust apps with GerenciaNet 🙂

## How to use it:
  
1. Clone or download the repository.
2. GerenciaNet requires a certificate to be sent in every HTTP request, download your certificates from their platform.
3. Put the downloaded certificate files (`homologacao-xx.p12`, `producao-xx.p12`) in the `certificates` folder in the project's root folder.
4. Rename the file `local.yaml.example` to `local.yaml`.
5. Change the `credentials` information accordingly: `client_id`, `client_secret`, `certificado_pix`.
6. That's it! You can then open the terminal and type `cargo run`. If everything goes alright, the qrcode image will be saved as `qrcode.png` in the root folder.
