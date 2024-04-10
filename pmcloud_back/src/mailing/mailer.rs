use std::env;
use lazy_static::lazy_static;

use mail_send::mail_builder::MessageBuilder;
use mail_send::SmtpClientBuilder;
use serde::Serialize;
use tera::{Context, Tera};
use tokio::task;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("src/mailing/templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html"]);
        tera
    };
}

pub fn send_rendered_email<T>(to: (String, String), subject: String, body_text: String, template: String, context: T) where T: Serialize + Send + 'static {
    task::spawn(async move {
        let context = &Context::from_serialize(context).expect("Unable to serialize email context.");
        let html = render_email_context(template, context.clone());
        send_email_async(to, subject, body_text, html)
    });
}
pub fn render_email_context(template: String, context: Context) -> String {
    TEMPLATES.render(format!("{}.html", template).as_str(), &context)
            .expect("Unable to render email template.")
}

pub fn send_email(to: (String, String), subject: String, body_text: String, body_html: String) {
    task::spawn(send_email_async(to, subject, body_text, body_html));
}

pub async fn send_email_async(to: (String, String), subject: String, body_text: String, body_html: String) {
    let server: String = env::var("SMTP_SERVER").expect("SMTP_SERVER must be set");
    let username: String = env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
    let password: String = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");

    let message = MessageBuilder::new()
        .from(("PMCloud", "no-reply@pdf4teachers.org"))
        .to(vec![to.clone()])
        .subject(subject)
        .text_body(body_text)
        .html_body(body_html);

    let mut connect = SmtpClientBuilder::new(server, 587)
        .implicit_tls(false)
        .credentials((username, password))
        .connect()
        .await;

    if connect.is_err() {
        eprintln!("Failed to connect to SMTP server: {:?}", connect.err().unwrap());
    }else{
        if let Err(e) = connect.unwrap().send(message).await {
            eprintln!("Failed to send email: {:?}", e);
        }else{
            println!("Email sent successfully to: {} [{}]", to.0, to.1);
        }
    }
}
