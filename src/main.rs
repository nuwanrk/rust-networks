mod api;
mod snowcast;

fn main() {
    execute()
}

fn execute() {
    for arg in std::env::args() {
        if arg.eq("udp_server") {
            api::udp_server();
        } else if arg.eq("udp_client") {
            api::udp_client();
        }
    }
}
