use std::path::PathBuf;
use std::env;
use std::io;
use std::fs;
use std::fs::canonicalize;

#[derive(Eq, Debug, Clone, PartialEq)]
pub struct GamePaths {
    pub resources: String,
    pub openal : String,
    pub profile : PathBuf,
}

#[derive(Debug)]
pub enum PathError {
    IO(io::Error),
    NoHomeDir,
    NoResourcesDir,
}

impl From<io::Error> for PathError {
    fn from(err: io::Error) -> Self {
        PathError::IO(err)
    }
}

pub fn get_paths(prefix: &str) -> Result<GamePaths, PathError> {
    let resources = canonicalize("./resources")?;
    let resources_string = resources.to_str().ok_or(PathError::NoResourcesDir)?.to_string();

    if cfg!(all(target_os = "macos")) { 
        if cfg!(debug_assertions) { // debug, just use local
            Ok((GamePaths {
                resources: resources_string,
                openal: "./native/openal.dylib".into(),
                profile: PathBuf::from(format!("./{}.profile.json", prefix)), 
            }))
        } else {
            // mac in a .app
            let mut resources_path = env::current_exe().unwrap();
            resources_path.pop();
            resources_path.pop();
            resources_path.push("Resources");
            
            let r_path = resources_path.to_str().unwrap().into();

            let mut alpth = resources_path.clone();
            alpth.push("openal.dylib");

            let al_path = alpth.to_str().unwrap().into();

            // profile goes in app support
            let mut home_dir = try!(env::home_dir().ok_or(PathError::NoHomeDir));
            home_dir.push("Library");
            home_dir.push("Application Support");
            home_dir.push(prefix);

            if !home_dir.exists() {
                fs::create_dir(&home_dir)?;
            } 
            home_dir.push(format!("{}.profile.json", prefix));
            
            Ok((GamePaths {
                resources: r_path,
                openal: al_path,
                profile: home_dir, 
            }))
        }
    } else  {
        
        
        Ok((GamePaths {
            resources: resources_string,
            openal: "./native/OpenAL64.dll".into(),
            profile: PathBuf::from("./{}.profile.json"), // current directory
        }))
    }
}