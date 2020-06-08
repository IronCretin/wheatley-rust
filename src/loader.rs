#[cfg(not(target_arch = "wasm32"))]
use std::fs;
#[cfg(target_arch = "wasm32")]
use stdweb::js;

#[cfg(not(target_arch = "wasm32"))]
pub fn load(paths: &[&str], callback: Box<dyn Fn(Vec<String>) -> ()>) {
    let mut results = Vec::with_capacity(paths.len());
    for path in paths {
        results.push(fs::read_to_string("static/".to_owned() + path).unwrap());
    }
    callback(results);
}

#[cfg(target_arch = "wasm32")]
pub fn load(paths: &[&str], callback: Box<dyn Fn(Vec<String>) -> ()>) {
    js! {
        let paths = @{paths};
        let callback = @{callback};

        let xhrs = [];
        let resps = {};

        function add_req(path) {
            let xhr = new XMLHttpRequest();
            xhr.open("GET", path);
            xhr.addEventListener("load", () => {
                resps[path] = xhr.responseText;
                if (xhrs.every(x => x.readyState == 4)) {
                    let texts = [];
                    for (let path of paths) {
                        texts.push(resps[path]);
                    }
                    callback(texts);
                    callback.drop();
                }
            });
            xhrs.push(xhr);
        }
        for (let path of paths) {
            add_req(path);
        }

        for (let x of xhrs) {
            x.send();
        }
    }
}
