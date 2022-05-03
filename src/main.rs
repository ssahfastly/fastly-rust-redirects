use fastly::{Request, Response};
use serde_json;
use serde_json::Value;
use std::collections::HashMap;
use std::time::SystemTime;

static mut PATH_CHECKS: Option<HashMap<String, String>> = None;

#[export_name = "wizer.initialize"]
pub extern "C" fn init() {
    unsafe {
        // Init path checks
        let mut checks: HashMap<String, String> = HashMap::new();

        let path_rules_json: Value =
            serde_json::from_str(include_str!("./rules/path-match-rules.json")).unwrap();
        let path_rules = path_rules_json.as_array().unwrap();

        for entry in path_rules.into_iter() {
            let pair = entry.as_array().unwrap();
            checks.insert(
                pair[0].as_str().unwrap().to_string(),
                pair[1].as_str().unwrap().to_string(),
            );
        }

        PATH_CHECKS = Some(checks);
    };
}


#[export_name = "wizer.resume"]
unsafe fn resume() {
    let start_time = SystemTime::now();

    let req = Request::from_client();
    
    if let Some(matched_rule) = PATH_CHECKS.take().unwrap().get(req.get_path()) {
        respond(req, Some(matched_rule.clone()), start_time);
        return;
    }

    respond(req, None, start_time);
}

fn respond(req: Request, next_url: Option<String>, start_time: SystemTime) {

    let mut resp;

    if let Some(url) = next_url {
        println!("Redirecting to {}", url);
        resp = Response::redirect(url);
    } else {
        println!("No redirect");
        resp = req.send("origin").expect("Could not send response");
    }

    resp.set_header("x-redirect-lookup-ms", format!("{}", start_time.elapsed().unwrap().as_millis()));
    resp.set_header("x-fastly-pop", std::env::var("FASTLY_POP").unwrap_or_else(|_| String::new()));
    resp.set_header("x-fastly-region", std::env::var("FASTLY_REGION").unwrap_or_else(|_| String::new()));
    resp.set_header("x-fastly-traceid", std::env::var("FASTLY_TRACE_ID").unwrap_or_else(|_| String::new()));

    resp.send_to_client();
}

fn main() -> Result<(), fastly::Error> {
    println!("This doesn't run.");
    Ok(())
}