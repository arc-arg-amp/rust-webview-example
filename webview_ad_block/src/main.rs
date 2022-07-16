#![windows_subsystem = "windows"]

extern crate web_view;

use std::env;
use web_view::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let content = match args.len() {
        1 => Content::Url("https://google.com/"),
        _ => {
            if args[1].len() > 8 && args[1].starts_with("https://") {
                Content::Url(args[1].as_str())
            } else {
                Content::Html(HTML)
            }
        },
    };
    let mut webview = web_view::builder()
        .title("webview example")
        .content(content)
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .build()
        .unwrap();

    let interval = 100;
    let mut counter = 0;
    loop {
        if webview.step().is_none() {
            break;
        }
        if counter <= interval {
            if counter == interval {
                counter = 0;
                webview.eval(JS.trim()).unwrap();
            }
            counter += 1;
        }
    }
}

const HTML: &str = r#"
<!doctype html>
<html>
    <body>
        <h1>Test</h1>
        <span id="ad">Ad1</span>
        <span class="ad_overlay">Ad2</span>
        <span data-google-query-id="ad">Ad3</span>
    </body>
</html>
"#;

const JS: &str = r#"
var idList = ['ad'];
for(var i = 0; i < idList.length; i++) {
  const elm = document.getElementById(idList[i]);
  if (!!elm) elm.parentNode.removeChild(elm);
}
var classList = ['ad_overlay',
                 'ad_rectangle',
                 'ad_list_top',
                 'ad_topics_custom',
                 'ad_custom'];
for(var i = 0; i < classList.length; i++) {
  const elmList = document.getElementsByClassName(classList[i]);
  for(var j = 0; j < elmList.length; j++) {
    elmList[j].parentNode.removeChild(elmList[j]);
  }
}
var selectorList = ['data-google-query-id'];
for(var i = 0; i < selectorList.length; i++) {
  const elmList = document.querySelectorAll('[' + selectorList[i] + ']');
  for(var j = 0; j < elmList.length; j++) {
    elmList[j].parentNode.removeChild(elmList[j]);
  }
}
"#;
