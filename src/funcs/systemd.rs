use super::Runner;
use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use dbus::blocking::Connection;
use serde::Deserialize;
use std::fmt;
use std::time::Duration;
#[derive(Deserialize, Debug)]
enum State {
    Started,
    Restarted,
    Enabled,
    Disabled,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize)]
pub struct Systemd {
    name: String,
    service: String,
    state: State,
}
// TODO: Error Handling needs to be generic over a custom error instead to avoid unwraping
// complete the systemd module by implementing the State enum variants
#[typetag::deserialize]
impl Runner for Systemd {
    fn run(&mut self) -> Result<(), std::io::Error> {
        println!("TASK: {}", self.name);
        let conn = Connection::new_system().unwrap();
        let proxy = conn.with_proxy(
            "org.freedesktop.systemd1",
            "/org/freedesktop/systemd1",
            Duration::from_millis(5000),
        );
        let (state,): (String,) = proxy
            .method_call(
                "org.freedesktop.systemd1.Manager",
                "GetUnitFileState",
                (format!("{}.service", self.service),),
            )
            .unwrap();
        println!("{state}");
        if state == self.state.to_string().to_lowercase() {
            println!("SERVICE IS {state}");
            return Ok(());
        }
        Ok(())
    }
}
