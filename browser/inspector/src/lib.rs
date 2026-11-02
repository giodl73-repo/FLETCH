use fletch_registry::{
    registry_index_from_registries, search_registry_index, validate_registry, FletchRegistry,
};
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use wasm_bindgen::prelude::*;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    registry_json: String,
    #[serde(default)]
    text: String,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    metadata_filters: Vec<(String, String)>,
    #[serde(default)]
    offset: u32,
    #[serde(default = "default_limit")]
    limit: u32,
}
fn default_limit() -> u32 {
    100
}
fn error(class: &str, reason: impl ToString) -> Value {
    json!({"schema_version":"fletch.browser-inspection.v1", "result":class, "error":reason.to_string(), "validation":null, "search":null})
}
fn run(input: &str) -> Value {
    if input.len() > 3 * 1024 * 1024 {
        return error(
            "browser_refusal",
            "Request exceeds 3 MiB browser framing limit",
        );
    }
    let request: Request = match serde_json::from_str(input) {
        Ok(value) => value,
        Err(e) => return error("invalid_input", e),
    };
    if request.registry_json.len() > 1024 * 1024
        || request.text.len() > 1024
        || request.tags.len() > 30
        || request.metadata_filters.len() > 30
        || request.limit == 0
        || request.limit > 100
        || request.tags.iter().any(|s| s.len() > 1024)
        || request
            .metadata_filters
            .iter()
            .any(|(k, v)| k.len() > 1024 || v.len() > 1024)
    {
        return error("browser_refusal", "Browser limits: registry 1 MiB; query/filter strings 1024 bytes; 30 tags/pairs; page 1..100");
    }
    let registry: FletchRegistry = match serde_json::from_str(&request.registry_json) {
        Ok(value) => value,
        Err(e) => return error("invalid_input", e),
    };
    let definitions = registry.fletches.len();
    let shafts: usize = registry.fletches.iter().map(|d| d.shafts.len()).sum();
    let edges: usize = registry.fletches.iter().map(|d| d.edges.len()).sum();
    if definitions > 500 || shafts > 4000 || edges > 5000 {
        return error(
            "browser_refusal",
            "Browser limits: 500 fletches, 4000 shafts, 5000 edges",
        );
    }
    let validation = validate_registry(&registry);
    let search = if validation.valid {
        let index = registry_index_from_registries(std::slice::from_ref(&registry));
        Some(search_registry_index(
            &index,
            &request.tags,
            &request.metadata_filters,
            if request.text.is_empty() {
                None
            } else {
                Some(&request.text)
            },
            request.offset as usize,
            Some(request.limit as usize),
        ))
    } else {
        None
    };
    json!({"schema_version":"fletch.browser-inspection.v1", "result":if validation.valid{"ok"}else{"invalid_registry"}, "registry_id":registry.registry_id, "supplied_utf8_sha256":format!("{:x}",Sha256::digest(request.registry_json.as_bytes())), "validation":validation,"search":search,"error":null})
}
#[wasm_bindgen]
pub fn inspect_registry(input: &str) -> String {
    let output = run(input).to_string();
    if output.len() > 4 * 1024 * 1024 {
        error("browser_refusal", "Report exceeds 4 MiB browser limit").to_string()
    } else {
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn request(registry: Value) -> String {
        json!({"registry_json":registry.to_string()}).to_string()
    }
    fn registry() -> Value {
        json!({"schema_version":"fletch.registry.v1","generated_by":"authored example","registry_id":"example","fletches":[{"id":"source:a","node_kind":"fletch","shafts":[{"kind":"file","url":"local-a","headers":{}}],"tags":["demo"],"metadata":{"year":"2026"}}]})
    }
    #[test]
    fn valid_search_is_derived_and_retains_native_label() {
        let v = run(&request(registry()));
        assert_eq!(v["result"], "ok");
        assert_eq!(v["search"]["generated_by"], "fletch-core/0.1.0");
        assert_eq!(v["search"]["matched_row_count"], 1);
    }
    #[test]
    fn invalid_registry_never_indexes() {
        for change in [0, 1, 2] {
            let mut r = registry();
            if change == 0 {
                r["schema_version"] = json!("wrong");
            } else if change == 1 {
                r["fletches"][0]["shafts"] = json!([]);
            } else {
                let d = r["fletches"][0].clone();
                r["fletches"].as_array_mut().unwrap().push(d);
            }
            let v = run(&request(r));
            assert_eq!(v["result"], "invalid_registry");
            assert!(v["search"].is_null());
        }
    }
    #[test]
    fn native_query_semantics_and_pagination() {
        let mut q = json!({"registry_json":registry().to_string(),"tags":["demo"],"metadata_filters":[["year","2026"]],"text":"SOURCE A"});
        assert_eq!(run(&q.to_string())["search"]["matched_row_count"], 1);
        q["tags"] = json!(["DEMO"]);
        assert_eq!(run(&q.to_string())["search"]["matched_row_count"], 0);
        q["tags"] = json!([]);
        q["offset"] = json!(1);
        let v = run(&q.to_string());
        assert_eq!(v["search"]["matched_row_count"], 1);
        assert_eq!(v["search"]["rows"], json!([]));
    }
    #[test]
    fn admission_is_separate() {
        assert_eq!(run("{")["result"], "invalid_input");
        assert_eq!(
            run(&json!({"registry_json":registry().to_string(),"limit":0}).to_string())["result"],
            "browser_refusal"
        );
        assert_eq!(
            run(&json!({"registry_json":" ".repeat(1024*1024+1)}).to_string())["result"],
            "browser_refusal"
        );
    }
}
