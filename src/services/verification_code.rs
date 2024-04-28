
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rand::Rng;


/*
step 1:  turn on 2 factor authorization of your gmail
step 2: go to this link , and generate app password     ---- https://myaccount.google.com/apppasswords?rapt=AEjHL4MKMoxNci-OoCa7aiPPhp_943zVvKWaPxjtVkheVjR2SqAT2f_mcAdaffIOVrHIwTJBE9gWxJF-ItDaZewJHssqfRUEpjpHxjvYkgex3TOc8Bv-xQc
step 3: copy app passwords
*/ 


pub fn email_sent(user_email:String,verify_code: String) {
    let email_body = format!(
        "Hello,\n\n\
        Your verification code for Gmail is: {}  \n\n\
        Please use this code to verify your account.\n\n\
        Regards,\n\
        School",verify_code
        
    );


    // compose a email
    let email = Message::builder()
        .from("".parse().unwrap())// TODO: write here your email address
        .to(user_email.parse().unwrap())
        .subject("Please, Verify Your Account")
        .body(email_body)
        .unwrap();

    let creds = Credentials::new(
        String::from(""),//TODO: write here your email address
        String::from(""),//TODO: paste your app password
    );

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully"),
        Err(err) => println!("could not sent the email {:?}", err),
    }
}


pub fn generate_otp()->String{
    let max_digits = 6;
    let mut rng = rand::thread_rng();
    
    // Calculate the maximum value for a 6-digit number (999999)
    let max_value = 10u32.pow(max_digits) - 1; // 10^6 - 1 = 999999
    
    // Generate a random number between 0 and the maximum value (999999)
    let random_number: u32 = rng.gen_range(0..=max_value);
    
   random_number.to_string()
}