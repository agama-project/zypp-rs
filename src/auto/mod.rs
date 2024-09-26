// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

mod context;
pub use self::context::Context;

mod expected;
pub use self::expected::Expected;

mod info_base;
pub use self::info_base::InfoBase;

mod progress_observer;
pub use self::progress_observer::ProgressObserver;

mod repo_info;
pub use self::repo_info::RepoInfo;

mod repo_manager;
pub use self::repo_manager::RepoManager;

mod repository;
pub use self::repository::Repository;

mod service_info;
pub use self::service_info::ServiceInfo;

mod managed_file;
pub use self::managed_file::ManagedFile;

mod repo_manager_options;
pub use self::repo_manager_options::RepoManagerOptions;

mod enums;
pub use self::enums::Exception;
pub use self::enums::RepoInfoType;
pub use self::enums::RepoManagerError;
pub use self::enums::RepoRefreshResult;

pub(crate) mod traits {
    pub use super::info_base::InfoBaseExt;
}
pub(crate) mod builders {
    pub use super::context::ContextBuilder;
    pub use super::progress_observer::ProgressObserverBuilder;
    pub use super::repo_info::RepoInfoBuilder;
    pub use super::repo_manager::RepoManagerBuilder;
}
