extern crate csv;

use std::error::Error;
use std::io;
use std::process;
use std::fs::File;

const DOC: &str = r#"
parameters:
  key_name: "user"
  image: "image"
  flavor: "flavor"
  security_groups: [ "default" ]
  volume_size: 100
  volume_type: "SSD"
  vm_name: "vm"
  domain: "domain"
  az: "az"
  network:
    - network: "network"
      vnic_type: "direct"
      fixed_ip:
        - "IP"
"#;

fn make_template() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        let record = result?;
        let image = &record[1];
        let flavor = &record[2];
        let security_groups = &record[3];
        let volume_size = &record[4];
        let vm_name = &record[0];
        let domain = &record[5];
        let az = &record[6];
        let network = &record[7];
        let fixed_ip = &record[8];

        let mut value: serde_yaml::Value = serde_yaml::from_str(DOC).unwrap();
        value["parameters"]["image"] = image.into();
        value["parameters"]["flavor"] = flavor.into();
        value["parameters"]["security_groups"] = security_groups.into();
        value["parameters"]["volume_size"] = volume_size.into();
        value["parameters"]["vm_name"] = vm_name.into();
        value["parameters"]["domain"] = domain.into();
        value["parameters"]["az"] = az.into();
        value["parameters"]["network"][0]["network"] = network.into();
        value["parameters"]["network"][0]["fixed_ip"][0] = fixed_ip.into();

        let file = File::create(["./heat_templates/", "env_", vm_name, ".yaml"].concat()).unwrap();
        serde_yaml::to_writer(file, &value).unwrap();
    }
    Ok(())
}

fn main() {
    if let Err(err) = make_template() {
        println!("error running generator: {}", err);
    }
    process::exit(1);
}
