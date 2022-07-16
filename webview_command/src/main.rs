#![windows_subsystem = "windows"]
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::process::Command;
use web_view::*;

fn command_exec(cmd: &str, param: &str) -> String {
    let output = if cfg!(target_os = "windows"){
        Command::new("cmd")
            .args(["/C", &format!("{} {}", cmd, param)])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(format!("{} {}", cmd, param))
            .output()
            .expect("failed to execute process")
    };
    String::from_utf8_lossy(output.stdout.as_slice()).to_string()
}

fn main() {
    web_view::builder()
        .title("Command Example")
        .content(Content::Html(HTML))
        .size(825, 625)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, arg| {
            use Cmd::*;
            match serde_json::from_str(arg).unwrap() {
                Ls { param } => {
                    let cmd = if cfg!(target_os = "windows"){
                        "dir"
                    } else {
                        "ls"
                    };
                    let result = command_exec(cmd, param.as_str());
                    if result.is_empty() {
                        println!("result is empty");
                    } else {
                        let eval_str = format!("LoadTextArea(\"{}\");", result.as_str().escape_default()).replace("\\u", "");
                        webview.eval(&eval_str)?;
                    }
                },
                Ps { param } => {
                    let cmd = if cfg!(target_os = "windows"){
                        "tasklist"
                    } else {
                        "ps"
                    };
                    let result = command_exec(cmd, param.as_str());
                    if result.is_empty() {
                        println!("result is empty");
                    } else {
                        let eval_str = format!("LoadTextArea(\"{}\");", result.as_str().escape_default()).replace("\\u", "");
                        webview.eval(&eval_str)?;
                    }
                },
            }
            Ok(())
        })
        .run()
        .unwrap();
}

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    Ls { param: String },
    Ps { param: String },
}

const HTML: &str = r#"
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
      'use strict';
      var rpc = {
        invoke : function(arg) { window.external.invoke(JSON.stringify(arg)); },
        ls : function() {
          var param = document.getElementById("param");
          var textArea = document.getElementById("text_box");
          textArea.value = "";
          rpc.invoke({cmd : 'ls', param : param.value.trim()});
        },
        ps : function() {
          var param = document.getElementById("param");
          var textArea = document.getElementById("text_box");
          textArea.value = "";
          rpc.invoke({cmd : 'ps', param : param.value.trim()});
        },
      };
        
      function LoadTextArea(data) {
        var textArea = document.getElementById("text_box");
        textArea.value = data;
      }
    </script>
  </head>
  <body>
    <label for="param">Param for command:</label>
    <input style="font-size:13px" id="param" type="text" size="10" />
    <button onclick="rpc.ls()">Ls</button>
    <button onclick="rpc.ps()">Ps</button>
    <textarea class="textarea" id="text_box"></textarea>
  </body>
</html>
"#;
