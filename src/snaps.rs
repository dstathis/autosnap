use toml::Value;
use virt::connect::Connect;

pub fn do_snaps(conf: &Value) {
    let flags = virt::connect::VIR_CONNECT_LIST_DOMAINS_ACTIVE |
                virt::connect::VIR_CONNECT_LIST_DOMAINS_INACTIVE;
    let uris = conf["main"]["uris"].as_str().expect("No configured URIs").split_whitespace();
    for uri in uris {
         let conn = match Connect::open(&uri) {
             Ok(c)  => c,
             Err(e) => {
                 panic!("No connection to hypervisor: code {}, message: {}",
                        e.code,
                        e.message);
             }
         };
         println!("{}:", uri);
         let doms = match conn.list_all_domains(flags) {
             Ok(c)  => c,
             Err(e) => {
                 panic!("Can't get domains from hypervisor: code {}, message: {}",
                        e.code,
                        e.message);
             }
         };
         for dom in doms {
             println!("{}", dom.get_name().unwrap());
         }
    }
}
