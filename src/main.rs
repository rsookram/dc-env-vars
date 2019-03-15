extern crate yaml_rust;

use yaml_rust::{Yaml, YamlLoader};

fn main() {
    let service_option = std::env::args().nth(1);
    let service = match service_option {
        Some(ref s) => s.as_str(),
        None => return,
    };

    let data_string = std::fs::read_to_string("docker-compose.yml")
        .expect("failed to read docker-compose.yml");
    let data_str = data_string.as_str();
    let docs = YamlLoader::load_from_str(data_str)
        .expect("docker-compose.yml doesn't contain valid YAML");

    let doc = &docs[0];

    let env = &doc["services"][service]["environment"];
    match env {
        yaml_rust::Yaml::Array(a) => {
            for e in a {
                print!("{} ", format_node(e));
            }
        }
        yaml_rust::Yaml::Hash(h) => {
            for (k, v) in h {
                print!("{}={} ", format_node(k), format_node(v));
            }
        }
        _ => panic!("Unexpected value for environment {:?}", env),
    }
    println!();
}

fn format_node(n: &Yaml) -> String {
    use yaml_rust::Yaml::*;

    return match n {
        Real(s) => s.to_string(),
        Integer(i) => format!("{}", i),
        String(s) => s.to_string(),
        Boolean(b) => format!("{}", b),
        Null => "null".to_string(),
        _ => panic!("Unexpected YAML node {:?}", n),
    };
}
