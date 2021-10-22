#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "AI")]
pub mod AI;
#[cfg(feature = "ApplicationModel")]
pub mod ApplicationModel;
#[cfg(feature = "Data")]
pub mod Data;
#[cfg(feature = "Devices")]
pub mod Devices;
#[cfg(feature = "Embedded")]
pub mod Embedded;
#[cfg(feature = "Foundation")]
pub mod Foundation;
#[cfg(feature = "Gaming")]
pub mod Gaming;
#[cfg(feature = "Globalization")]
pub mod Globalization;
#[cfg(feature = "Graphics")]
pub mod Graphics;
#[cfg(feature = "Management")]
pub mod Management;
#[cfg(feature = "Media")]
pub mod Media;
#[cfg(feature = "Networking")]
pub mod Networking;
#[cfg(feature = "Perception")]
pub mod Perception;
#[cfg(feature = "Phone")]
pub mod Phone;
#[cfg(feature = "Security")]
pub mod Security;
#[cfg(feature = "Services")]
pub mod Services;
#[cfg(feature = "Storage")]
pub mod Storage;
#[cfg(feature = "System")]
pub mod System;
#[cfg(feature = "UI")]
pub mod UI;
#[cfg(feature = "Web")]
pub mod Web;
#[cfg(feature = "Win32")]
pub mod Win32;