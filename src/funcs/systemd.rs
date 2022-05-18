use super::Runner;
use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use dbus::blocking::Connection;
use serde::Deserialize;
use std::fmt;
use std::time::Duration;
use dbus::strings::Path;

use dbus::blocking::stdintf::org_freedesktop_dbus::ObjectManager;




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
        let mut proxy = conn.with_proxy(
            "org.freedesktop.systemd1",
            "/org/freedesktop/systemd1",
            Duration::from_millis(5000),
        );
        let (e_state,): (String,) = proxy
            .method_call(
                "org.freedesktop.systemd1.Manager",
                "GetUnitFileState",
                (format!("{}.service", self.service),),
            )
            .unwrap();
        eprintln!("{e_state}");
        let unit_path: (Path,)= proxy
            .method_call(
                "org.freedesktop.systemd1.Manager",
                "GetUnit",
                (format!("{}.service", self.service),),
            )
            .unwrap();
        proxy = conn.with_proxy( "org.freedesktop.systemd1", unit_path.0,  Duration::from_millis(5000),);

       let active_state: String = proxy.get("org.freedesktop.systemd1.Unit", "ActiveState").unwrap();
       println!("ACTIVE {}", active_state);
       let sub_state: String = proxy.get("org.freedesktop.systemd1.Unit", "SubState").unwrap();
        println!("SUB STATE {sub_state}");
        if e_state == self.state.to_string().to_lowercase() {
            println!("SERVICE IS {e_state}");
            return Ok(());
        }
        
        match self.state {
        State::Started => {
            if e_state == "disabled" {
                // enable the service first
                todo!()
            }
            // start the service here
            
        }
        State::Restarted => {}
        State::Enabled => {}
        State::Disabled => {}
        }
        Ok(())
    }
}
