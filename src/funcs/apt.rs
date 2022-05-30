use super::opts::Opts;
use super::runner::TError;
use super::Runner;
use serde::Deserialize;
use std::process::Command;

/// The state for the apt package, currently TRun supports Present and Removed Options.
/// TRun will not install a package that has already been installed, or removed.
#[derive(Deserialize, PartialEq)]
pub enum State {
    /// Aliases: Installed
    #[serde(alias = "Installed")]
    Present,
    /// Aliases: Uninstalled
    #[serde(alias = "Uninstalled")]
    Removed,
}

/// The apt struct uses the same structure as other modules. To use this struct in TOML file the
/// user has to specify a unique name for the task, a valid application name, and the state from
/// the State Enum.
/// Optionally, it is possible to add any optional parameters such as debug and panics which
/// allows the user to control the flow of the deployment.
/// # Example Toml File:
/// ```toml
/// ["Install CowSay"]
/// module = "Apt"
/// name = "Install CowSay"
/// app = "cowsay"
/// state = "Removed"
/// panics = false
/// ```
/// In this example, TRun will make sure that cowsay is uninstalled from the system and not panic
/// when it finds any error in the process (such as not having apt in the system to begin with).
#[derive(Deserialize)]
pub struct Apt {
    name: String,
    app: String,
    state: State,
    #[serde(flatten)]
    opts: Opts,
}

impl Runner for Apt {
    fn run(&mut self) -> Result<(), TError> {
        println!("TASK {}", self.name);
        // Check if Apt is installed first
        let output = Command::new("apt")
            .args(["-qq", "list", &self.app])
            .output()?;

        let state = {
            if !std::str::from_utf8(&output.stdout)?
                .lines()
                .map(|l| return l.split_whitespace().last().unwrap().contains("[installed]"))
                .any(|p| p)
            {
                State::Removed
            } else {
                State::Present
            }
        };

        match self.state {
            State::Present => {
                if state == State::Removed {
                    let install = Command::new("apt")
                        .args(["install", "-y", &self.app])
                        .output()?;
                    if install.status.code().unwrap() != 0 {
                        println!("Error Installing The Package");
                        println!("{}", std::str::from_utf8(&install.stderr)?);
                        return Ok(());
                    }
                    println!("Package is installed successfully");
                    return Ok(());
                }
                println!("Package is already installed");
                Ok(())
            }
            State::Removed => {
                if state == State::Present {
                    let uninstall = Command::new("apt")
                        .args(["remove", "-y", &self.app])
                        .output()?;
                    if uninstall.status.code().unwrap() != 0 {
                        println!("Error Uninstalling The Package");
                        return Ok(());
                    }
                    println!("Package is Removed successfully");
                    return Ok(());
                }
                println!("Package doesn't exist to be uninstalled");
                Ok(())
            }
        }
    }

    fn panics(&self) -> bool {
        if let Some(x) = self.opts.panics {
            return x;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "apt")]
    fn apt_installed() {
        use super::*;
        let mut apt = Apt {
            name: "Test Apt".to_owned(),
            state: State::Present,
            app: "apt".to_owned(),
            opts: Default::default(),
        };
        assert!(apt.run().is_ok());
    }
}
