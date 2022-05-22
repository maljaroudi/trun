use super::runner::TError;
use super::Runner;

use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use dbus::blocking::Connection;
use dbus::strings::Path;
use serde::Deserialize;
use std::fmt;
use std::time::Duration;
#[derive(Deserialize, Debug)]
enum State {
    Started,
    Stopped,
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
    enabled: Option<bool>,
}
//TODO: complete the systemd module by implementing the State enum variants
#[typetag::deserialize]
impl Runner for Systemd {
    fn run(&mut self) -> Result<(), TError> {
        println!("TASK: {}", self.name);
        let conn = Connection::new_system().map_err(TError::DbusError)?;
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
            .expect("ERROR, SERIVCE DOES NOT EXIST");

        println!("{e_state}");
        //let sub_state: String = proxy.get("org.freedesktop.systemd1.Unit", "SubState").unwrap();
        // println!("SUB STATE {sub_state}");
        if e_state == self.state.to_string().to_lowercase() {
            println!("SERVICE IS {e_state}");
            return Ok(());
        }
        match self.state {
            State::Started => {
                if e_state == "disabled" && self.enabled.unwrap_or(false) {
                    let (carries_install_info, _changes): (bool, Vec<String>) = proxy
                        .method_call(
                            "org.freedesktop.systemd1.Manager",
                            "EnableUnitFiles",
                            (vec![format!("{}.service", self.service)], false, true),
                        )
                        .map_err(TError::DbusError)?;
                    if !carries_install_info {
                        println!(
                        "FAILED TO ENABLE THE UNIT FILE: THE UNIT FILE LACKS THE INSTALL SECTION"
                    );
                    }
                    let _reload: () = proxy
                        .method_call("org.freedesktop.systemd1.Manager", "Reload", ())
                        .map_err(TError::DbusError)?;
                }

                let unit_path: (Path,) = proxy
                    .method_call(
                        "org.freedesktop.systemd1.Manager",
                        "LoadUnit",
                        (format!("{}.service", self.service),),
                    )
                    .map_err(TError::DbusError)?;

                proxy = conn.with_proxy(
                    "org.freedesktop.systemd1",
                    unit_path.0,
                    Duration::from_millis(5000),
                );

                let active_state: String = proxy
                    .get("org.freedesktop.systemd1.Unit", "ActiveState")
                    .map_err(TError::DbusError)?;
                println!("ACTIVE {}", active_state);

                if active_state == "active" {
                    println!("SERVICE IS {active_state} ACTION IS NOT REQUIRED");
                    return Ok(());
                }
                let _start: (Path,) = proxy
                    .method_call("org.freedesktop.systemd1.Unit", "Start", ("replace",))
                    .map_err(TError::DbusError)?;
                println!("SERVICE HAS STARTED");
                // TODO: Make sure that the service has fully started rather
                // than just sending the command
            }
            State::Stopped => {
                let unit_path: Result<(Path,), dbus::Error> = proxy.method_call(
                    "org.freedesktop.systemd1.Manager",
                    "LoadUnit",
                    (format!("{}.service", self.service),),
                );
                if let Ok(unit_path) = unit_path {
                    proxy = conn.with_proxy(
                        "org.freedesktop.systemd1",
                        unit_path.0,
                        Duration::from_millis(5000),
                    );

                    let active_state: String = proxy
                        .get("org.freedesktop.systemd1.Unit", "ActiveState")
                        .map_err(TError::DbusError)?;
                    println!("ACTIVE {}", active_state);
                    if active_state == "inactive" {
                        println!("SERVICE IS {active_state} ACTION IS NOT REQUIRED");
                        return Ok(());
                    }
                    let _stop: (Path,) = proxy
                        .method_call("org.freedesktop.systemd1.Unit", "Stop", ("replace",))
                        .unwrap();
                    println!("SERVICE HAS STOPPED");
                    return Ok(());
                }
                println!("Service is Already Stopped");
                return Ok(());
            }
            State::Restarted => {
                if e_state == "disabled" && self.enabled.unwrap_or(false) {
                    let (carries_install_info, _changes): (bool, Vec<String>) = proxy
                        .method_call(
                            "org.freedesktop.systemd1.Manager",
                            "EnableUnitFiles",
                            (vec![format!("{}.service", self.service)], false, true),
                        )
                        .map_err(TError::DbusError)?;
                    if !carries_install_info {
                        println!(
                        "FAILED TO ENABLE THE UNIT FILE: THE UNIT FILE LACKS THE INSTALL SECTION"
                    );
                    }
                    let _reload: () = proxy
                        .method_call("org.freedesktop.systemd1.Manager", "Reload", ())
                        .map_err(TError::DbusError)?;
                }

                let unit_path: (Path,) = proxy
                    .method_call(
                        "org.freedesktop.systemd1.Manager",
                        "LoadUnit",
                        (format!("{}.service", self.service),),
                    )
                    .map_err(TError::DbusError)?;
                proxy = conn.with_proxy(
                    "org.freedesktop.systemd1",
                    unit_path.0,
                    Duration::from_millis(5000),
                );

                let active_state: String = proxy
                    .get("org.freedesktop.systemd1.Unit", "ActiveState")
                    .map_err(TError::DbusError)?;
                println!("ACTIVE {}", active_state);

                let _restart: (Path,) = proxy
                    .method_call("org.freedesktop.systemd1.Unit", "Restart", ("replace",))
                    .map_err(TError::DbusError)?;
                println!("SERVICE HAS RESTARTED");
            }
            State::Enabled => {
                let (carries_install_info, _changes): (bool, Vec<String>) = proxy
                    .method_call(
                        "org.freedesktop.systemd1.Manager",
                        "EnableUnitFiles",
                        (vec![format!("{}.service", self.service)], false, true),
                    )
                    .map_err(TError::DbusError)?;
                if !carries_install_info {
                    println!(
                        "FAILED TO ENABLE THE UNIT FILE: THE UNIT FILE LACKS THE INSTALL SECTION"
                    );
                }
                let _reload: () = proxy
                    .method_call("org.freedesktop.systemd1.Manager", "Reload", ())
                    .map_err(TError::DbusError)?;
            }
            State::Disabled => {
                if let Some(enabled) = self.enabled {
                    if enabled {
                        println!(
                            "CAN'T HAVE BOTH ENABLED AND DISABLED INSTRUCTIONS AT THE SAME TIME"
                        );
                        return Ok(());
                    }
                }
                let _job: (Path,) = proxy
                    .method_call(
                        "org.freedesktop.systemd1.Manager",
                        "StopUnit",
                        (format!("{}.service", self.service), "replace"),
                    )
                    .map_err(TError::DbusError)?;
                let _changes: (Vec<String>,) = proxy
                    .method_call(
                        "org.freedesktop.systemd1.Manager",
                        "DisableUnitFiles",
                        (vec![format!("{}.service", self.service)], false),
                    )
                    .map_err(TError::DbusError)?;
                let _reload: () = proxy
                    .method_call("org.freedesktop.systemd1.Manager", "Reload", ())
                    .map_err(TError::DbusError)?;

                println!("SERVICE {} HAS BEEN DISABLED AND STOPPED", self.service);
            }
        }
        Ok(())
    }
}
