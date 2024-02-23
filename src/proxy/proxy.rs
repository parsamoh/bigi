enum ProxyProto {
    Vless,
    Vmess,
    Trojan,
    Shadowsocks,
    Socks,
}

enum TransportType {
    TCP,
    HTTP,
    H2,
    WS,
    GRPC,
}

struct ConfData {
    protocol: ProxyProto,
    id: String,
    url: String,
    port: u16,
    tls: bool,
    encryption: String,
    service_name: String,
    profile: String,
    sni: String,
    label: String,
}

// impl ConfData {
//     fn new(
//         protocol: ProxyProto,
//         id: String,
//         url: String,
//         port: u16,
//         encryption: String,
//         type: ,
//         service_name: String,
//         profile: String,
//         sni: String,
//         label: String,
//     ) -> Self {
//         Self {
//             protocol,
//             id,
//             url,
//             port,
//             encryption,
//             service_name,
//             profile,
//             sni,
//             label: label,
//         }
//     }

//     fn to_string(&self) -> String {
//         match self.protocol {
//             ProxyProto::Vless => format!(
//                 "vless://{}@{}:{}?encryption={}&type=grpc&security=tls&serviceName={}&fp=chrome&sni={}#{}",
//                 self.id, self.url, self.port, self.encryption, self.service_name, self.sni, self.label
//             ),
//             ProxyProto::Vmess => format!(
//                 "vmess://{}@{}:{}?encryption={}&type=grpc&security=tls&serviceName={}&fp=chrome&sni={}#{}",
//                 self.id, self.url, self.port, self.encryption, self.service_name, self.sni, self.label
//             ),
//             ProxyProto::Trojan => format!(
//                 "trojan://{}@{}:{}?encryption={}&type=grpc&security=tls&serviceName={}&fp=chrome&sni={}#{}",
//                 self.id, self.url, self.port, self.encryption, self.service_name, self.sni, self.label
//             ),
//             ProxyProto::Shadowsocks => format!(
//                 "ss://{}@{}:{}?encryption={}&type=grpc&security=tls&serviceName={}&fp=chrome&sni={}#{}",
//                 self.id, self.url, self.port, self.encryption, self.service_name, self.sni, self.label
//             ),
//             ProxyProto::Socks => format!(
//                 "socks://{}@{}:{}?encryption={}&type=grpc&security=tls&serviceName={}&fp=chrome&sni={}#{}",
//                 self.id, self.url, self.port, self.encryption, self.service_name, self.sni, self.label
//             ),
//         }
//     }
// }
