extern crate reqwest;
extern crate serde_json;

fn main() {
    let body = fetch_body();
    format(body);
}

fn fetch_body() -> String {
    let client = reqwest::blocking::Client::new();
    let agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/88.0.4324.104 Safari/537.36";
    let url = "https://www.google.com/search?tbm=isch&q=cats";

    let resp = client.get(url).header("User-Agent", agent).send().unwrap();
    resp.text().unwrap()
}

fn format(mut body: String) {
    let script = body.rfind("AF_initDataCallback").unwrap();
    body = body[script..].to_string();

    let start = body.find("[").unwrap();
    body = body[start..].to_string();

    let script_end = body.find("</script>").unwrap();
    body = body[..script_end].to_string();

    let end = body.rfind(",").unwrap();
    body = body[..end].to_string();

    let json: serde_json::Value = serde_json::from_str(&body).unwrap();

    std::fs::File::create("images.json").unwrap();
    std::fs::write("images.json", serde_json::to_string_pretty(&json).unwrap()).unwrap();
}
