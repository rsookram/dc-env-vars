extern crate yaml_rust;

use yaml_rust::{Yaml, YamlLoader};

const FILE_NAME: &str = "docker-compose.yml";

fn main() {
    let service_option = std::env::args().nth(1);
    let service = match service_option {
        Some(ref s) => s.as_str(),
        None => return,
    };

    let doc = load_yaml_doc(FILE_NAME);

    let env_vars = extract_env_vars(doc, service);

    println!("{}", env_vars.join(" "));
}

fn load_yaml_doc(name: &str) -> Yaml {
    let data_string = std::fs::read_to_string(name)
        .expect(&format!("failed to read {}", name));
    let data_str = data_string.as_str();
    let mut docs = YamlLoader::load_from_str(data_str)
        .expect(&format!("{} doesn't contain valid YAML", name));

    docs.pop().unwrap()
}

fn extract_env_vars(doc: Yaml, service: &str) -> Vec<String> {
    let env = &doc["services"][service]["environment"];

    match env {
        yaml_rust::Yaml::Array(a) =>
            a.iter()
                .map(|n| format_node(n))
                .collect(),
        yaml_rust::Yaml::Hash(h) =>
            h.iter()
                .map(|(k, v)| format!("{}={}", format_node(k), format_node(v)))
                .collect(),
        _ => panic!("Unexpected value for environment {:?}", env),
    }
}

fn format_node(n: &Yaml) -> String {
    use yaml_rust::Yaml::*;

    match n {
        Real(s) => s.to_string(),
        Integer(i) => format!("{}", i),
        String(s) => s.to_string(),
        Boolean(b) => format!("{}", b),
        Null => "null".to_string(),
        _ => panic!("Unexpected YAML node {:?}", n),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use yaml_rust::YamlLoader;

    #[test]
    #[should_panic]
    fn test_missing_environment() {
        let s = r#"---
version: "3"
services:
  api:
    build: .
"#;
        let mut out = YamlLoader::load_from_str(s).unwrap();
        let doc = out.pop().unwrap();

        extract_env_vars(doc, "api");
    }

    #[test]
    fn test_array_environment() {
        let s = r#"---
version: "3"
services:
  api:
    build: .
    environment:
      - DB_USERNAME=root
      - PORT=80
"#;
        let mut out = YamlLoader::load_from_str(s).unwrap();
        let doc = out.pop().unwrap();

        let vars = extract_env_vars(doc, "api");

        assert_eq!(vars, vec!["DB_USERNAME=root", "PORT=80"])
    }

    #[test]
    fn test_hash_environment() {
        let s = r#"---
version: "3"
services:
  api:
    build: .
    environment:
      DB_USERNAME: root
      PORT: 80
"#;
        let mut out = YamlLoader::load_from_str(s).unwrap();
        let doc = out.pop().unwrap();

        let vars = extract_env_vars(doc, "api");

        assert_eq!(vars, vec!["DB_USERNAME=root", "PORT=80"])
    }
}
