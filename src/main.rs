extern crate reqwest;
extern crate serde_json;

#[cfg(target_os = "windows")]
use winrt_notification::{Toast, Scenario, ToastWithHandlers, Duration, Sound, IconCrop};
#[cfg(target_os = "windows")]
use std::process::Command;

fn main() {
    let body = fetch_body();
    let json = unpack(body);
    format(&json);

    if cfg!(windows) {
        notif(check(json));
    } else {
        match check(json) {
            Ok(i) => {
                println!("{}", i);
            },
            Err((line, column)) => {
                println!("Report on line {line}, column {column}");
                println!("Failed while unpacking {{{}}}", match line {
                    69 => "objects",
                    70 => "inner",
                    72 => "info",
                    73 => "url",
                    74 => "width",
                    75 => "height",
                    77 => "thumbnail",
                    78 => "source",
                    _ => "{unknown}"
                });
            }
        }
    }
}

fn fetch_body() -> String {
    let client = reqwest::blocking::Client::new();
    let agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/88.0.4324.104 Safari/537.36";
    let url = "https://www.google.com/search?tbm=isch&q=cats";

    let resp = client.get(url).header("User-Agent", agent).send().unwrap();
    resp.text().unwrap()
}

fn unpack(mut body: String) -> serde_json::Value {
    let script = body.rfind("AF_initDataCallback").unwrap();
    body = body[script..].to_string();

    let start = body.find("[").unwrap();
    body = body[start..].to_string();

    let script_end = body.find("</script>").unwrap();
    body = body[..script_end].to_string();

    let end = body.rfind(",").unwrap();
    body = body[..end].to_string();

    serde_json::from_str(&body).unwrap()
}

fn format(json: &serde_json::Value) {
    std::fs::File::create("images.json").unwrap();
    std::fs::write("images.json", serde_json::to_string_pretty(json).unwrap()).unwrap();
}

/// Unwrap or report
macro_rules! uor {
    ($opt: expr) => {
        match $opt {
            Some(v) => v,
            None => {return Err((line!(), column!()));}
        }
    }
}

struct Image {
    url: String,
    width: usize,
    height: usize,
    thumbnail: String,
    source: String
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Image {{
    url: {},
    width: {},
    height: {},
    thumbnail: {},
    source: {}
}}", self.url, self.width, self.height, self.thumbnail, self.source)
    }
}

fn check(json: serde_json::Value) -> Result<Image, (u32, u32)> {
    let image_objects = uor!(uor!(uor!(uor!(uor!(uor!(uor!(uor!(json.as_array())[56].as_array())[1].as_array())[0].as_array()).last()).as_array())[1].as_array())[0].as_array());
    let inner = uor!(uor!(uor!(uor!(uor!(image_objects[0].as_array())[0].as_array())[0].as_object())["444383007"].as_array())[1].as_array());

    let info = uor!(inner[3].as_array());
    let url = uor!(info[0].as_str()).to_owned();
    let width = uor!(info[2].as_i64()) as usize;
    let height = uor!(info[1].as_i64()) as usize;

    let thumbnail = uor!(uor!(inner[2].as_array())[0].as_str()).to_owned();
    let source = uor!(uor!(uor!(inner[9].as_object())["2003"].as_array())[2].as_str()).to_owned();
    
    Ok(Image {
        url,
        width,
        height,
        thumbnail,
        source
    })
}

#[cfg(target_os = "windows")]
fn notif(checked: Result<Image, (u32, u32)>) {
    match checked {
        Ok(i) => {
            println!("Successfully unpacked image: {}", i);
            Toast::new("Microsoft.VisualStudioCode")
                .title("Success while unpacking images!")
                .duration(Duration::Short)
                .icon(&std::env::current_dir().unwrap().join("icon.png"), IconCrop::Circular, "icon")
                .show()
                .unwrap();
        },
        Err((line, column)) => {
            println!("Report on line {line}, column {column}");
            let failure = match line {
                100 => "objects",
                101 => "inner",
                103 => "info",
                104 => "url",
                105 => "width",
                106 => "height",
                108 => "thumbnail",
                109 => "source",
                _ => "{unknown}"
            };
            println!("Failed while unpacking {{{}}}", failure);
            let not = Toast::new(Toast::POWERSHELL_APP_ID)
                .scenario(Scenario::Reminder)
                .title(&format!("Failed on line {line}, column {column} while unwraping {failure}"))
                .sound(Some(Sound::IM))
                .icon(&std::env::current_dir().unwrap().join("icon.png"), IconCrop::Circular, "icon");
            
                ToastWithHandlers::new(not)
                    .on_activate(|_| {
                        Command::new("cmd")
                            .arg("/C")
                            .arg("code C:\\Users\\Jett\\Documents\\Scripts\\auto_images")
                            .output()
                            .unwrap();
                        Ok(())
                    })
                    .show()
                    .unwrap();
        }
    }
}
