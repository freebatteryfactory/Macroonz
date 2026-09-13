mod fixture;
mod inventory;
mod type_contract;
mod types;

#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
mod configure;
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
mod formatting;
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
mod format_refusal;
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
mod format_query;
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
mod prepared;
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
mod stage_fixture;
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
mod staging;
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
mod stage_refusal;
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
mod stage_format;
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
mod destination_fixture;
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
mod destination;
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
mod install_fixture;
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
mod installation;
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
mod interruption;
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
mod recovery;
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
mod destination_refusal;
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
mod destination_links;
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
mod regeneration;
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
mod command_fixture;
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
mod command;
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
mod command_refusal;
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
mod stage_cache;
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
mod example;
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
mod adopter;
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
mod visibility_fixture;
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
mod visibility;
#[cfg(any(windows, target_os = "linux", target_os = "macos"))]
mod command_output;
