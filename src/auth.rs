use actix_web::{web, HttpResponse};
use std::sync::Arc;

const AUTH_COOKIE: &str = "hsk_auth";

pub fn login_page(error: bool) -> String {
    let err_html = if error { r#"<p class="error">Wrong password</p>"# } else { "" };
    format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Julien's Chinese Vocabulary - Login</title>
<style>
* {{ margin: 0; padding: 0; box-sizing: border-box; }}
body {{ font-family: system-ui, sans-serif; background: #0f0f1a; color: #eee;
  display: flex; align-items: center; justify-content: center; min-height: 100vh; }}
.login-box {{ background: #1a1a2e; padding: 40px; border-radius: 16px; width: 100%; max-width: 360px; text-align: center; }}
h1 {{ font-size: 1.8rem; margin-bottom: 8px;
  background: linear-gradient(135deg, #e94560, #ff6b81); -webkit-background-clip: text; -webkit-text-fill-color: transparent; }}
p {{ color: #888; margin-bottom: 24px; }}
input {{ width: 100%; padding: 14px; background: #232340; border: 1px solid #333; border-radius: 10px;
  color: #eee; font-size: 1rem; margin-bottom: 16px; text-align: center; }}
input::placeholder {{ color: #666; }}
button {{ width: 100%; padding: 14px; background: #e94560; color: white; border: none; border-radius: 10px;
  font-size: 1.1rem; cursor: pointer; }}
button:hover {{ background: #ff6b81; }}
.error {{ color: #e74c3c; margin-bottom: 12px; font-size: 0.9rem; }}
</style>
</head>
<body>
<div class="login-box">
  <h1>🀄 Julien's Chinese Vocabulary</h1>
  <p>Enter password to continue</p>
  {err_html}
  <form method="POST" action="/login">
    <input type="password" name="password" placeholder="Password" autofocus>
    <button type="submit">Enter</button>
  </form>
</div>
</body>
</html>"#)
}

pub async fn login_get() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(login_page(false))
}

#[derive(serde::Deserialize)]
pub struct LoginForm {
    password: String,
}

pub async fn login_post(
    form: web::Form<LoginForm>,
    secret: web::Data<Arc<String>>,
) -> HttpResponse {
    if form.password == ***secret {
        HttpResponse::SeeOther()
            .append_header(("Location", "/"))
            .cookie(
                actix_web::cookie::Cookie::build(AUTH_COOKIE, "1")
                    .path("/")
                    .http_only(true)
                    .max_age(actix_web::cookie::time::Duration::days(30))
                    .finish(),
            )
            .finish()
    } else {
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(login_page(true))
    }
}
