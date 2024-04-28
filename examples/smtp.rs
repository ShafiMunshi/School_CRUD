extern crate lettre;
extern crate lettre_email;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

// fn main() {
//     // Set up SMTP credentials and server information
//     let smtp_username = "your_smtp_username";
//     let smtp_password = "your_smtp_password";
//     let smtp_server = "smtp.yourserver.com:587"; // Update this with your SMTP server address

//     // Create SMTP client
//     let smtp_client = SmtpTransport::relay(smtp_server)
//         .unwrap()
//         .credentials(Credentials::new(smtp_username.to_string(), smtp_password.to_string()))
//         .build();

//     // Compose email
//     let email = Message::builder()
//         .from("sender@example.com".parse().unwrap())
//         .to("recipient@example.com".parse().unwrap())
//         .subject("Hello from Rust!")
//         .body("This is a test email sent from Rust.")
//         .unwrap();

//     // Send email
//     match smtp_client.send(&email) {
//         Ok(_) => println!("Email sent successfully!"),
//         Err(e) => eprintln!("Failed to send email: {:?}", e),
//     }
// }

fn main() {
    // compose a email
    let email = Message::builder()
        .from("shafimunshi111@gmail.com".parse().unwrap())
        .to("shafimunshi66@gmail.com".parse().unwrap())
        .subject("Test email from rust")
        .body(String::from("This is test email  , please don''t reply"))
        .unwrap();

    let creds = Credentials::new(
        String::from("shafimunshi111@gmail.com"),
        String::from("wqxq jkjr vdab sfqh"),
    );

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_)=>println!("Email sent successfully"),
        Err(err)=> println!("could not sent the email {:?}",err)
    }
}
