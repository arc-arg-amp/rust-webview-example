extern crate web_view;
mod data_scraper;

use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use web_view::*;

fn main() {
    let mutex = Arc::new(Mutex::new("".to_string()));

    let mut webview1 = web_view::builder()
        .title("Multi window example - Window 1")
        .content(Content::Html(HTML1))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data("".to_string())
        .invoke_handler(|webview, arg| {
            *webview.user_data_mut() = arg.trim().to_string();
            Ok(())
        })
        .build()
        .unwrap();

    let mut webview2 = web_view::builder()
        .title("Multi window example - Window 2")
        .content(Content::Html(HTML2))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .build()
        .unwrap();

    let rt = rt();
    let handle = rt.handle().clone();
    loop {
        if webview1.step().is_none() {
            break;
        }

        if webview2.step().is_none() {
            break;
        }

        match mutex.clone().try_lock_owned() {
            Ok(mut x) => {
                if !x.trim().is_empty() {
                    let js = format!("LoadTextArea('{}');", x.trim().replace("'", "\\'"));
                    webview2.eval(&js).unwrap();
                    *x = "".to_string();
                }
            },
            _ => (),
        }

        if !webview1.user_data_mut().as_str().is_empty() {
            let param = String::from(webview1.user_data_mut().as_str());
            let tmp_mutex = Arc::clone(&mutex);
            let join_handle = handle.spawn(async move {
                let data = data_scraper::search(param.as_str()).await;
                let mut lock = tmp_mutex.lock().await;
                *lock = data;
            });
            *webview1.user_data_mut() = "".to_string();

            if webview1.step().is_none() {
                break;
            }

            if webview2.step().is_none() {
                break;
            }

            rt.block_on(async move {
                let _ = join_handle.await;
            });
        }
    }
}

fn rt() -> Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
}

const HTML1: &str = r#"
<!doctype html>
<html>
  <body>
    <button onclick="external.invoke('https://www.youtube.com/')">top</button>
    <button onclick="external.invoke('https://www.youtube.com/feed/trending')">trend</button>
  </body>
</html>
"#;

const HTML2: &str = r#"
<!doctype html>
<html>
  <head>
    <style>
      .textarea {
        width: 100%;
        height: 30em;
        font-size: 1em;
      }
    </style>
    <script type="text/javascript">
      function LoadTextArea(data) {
        var textArea = document.getElementById("text_box");
        textArea.value = data;
      }
    </script>
  </head>
  <body>
    <textarea class="textarea" id="text_box"></textarea>
  </body>
</html>
"#;
