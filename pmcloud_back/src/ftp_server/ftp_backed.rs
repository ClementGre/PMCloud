use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use async_trait::async_trait;
use libunftp::auth::{DefaultUser, UserDetail};
use libunftp::storage::{Fileinfo, Metadata, Result, StorageBackend};
use crate::ftp_server::ftp_auth::PMUser;

#[derive(Debug)]
pub struct Vfs {}

#[derive(Debug)]
pub struct Meta {
    inner: std::fs::Metadata,
}

impl Vfs {
    pub(crate) fn new() -> Vfs { Vfs {} }
}

#[async_trait]
impl StorageBackend<PMUser> for Vfs {
    type Metadata = Meta;

    async fn metadata<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &PMUser,
        path: P,
    ) -> Result<Self::Metadata> {
        unimplemented!()
    }

    async fn md5<P: AsRef<Path> + Send + Debug>(&self, user: &PMUser, path: P) -> Result<String>
        where
            P: AsRef<Path> + Send + Debug,
    {
        unimplemented!()
    }

    async fn list<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &PMUser,
        path: P,
    ) -> Result<Vec<Fileinfo<PathBuf, Self::Metadata>>>
    {
        unimplemented!()
    }

    async fn get<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &PMUser,
        path: P,
        start_pos: u64,
    ) -> Result<Box<dyn tokio::io::AsyncRead + Send + Sync + Unpin>> {
        unimplemented!()
    }

    async fn put<
        P: AsRef<Path> + Send + Debug,
        R: tokio::io::AsyncRead + Send + Sync + Unpin + 'static,
    >(
        &self,
        user: &PMUser,
        input: R,
        path: P,
        start_pos: u64,
    ) -> Result<u64> {
        unimplemented!()
    }

    async fn del<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &PMUser,
        path: P,
    ) -> Result<()> {
        unimplemented!()
    }

    async fn mkd<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &PMUser,
        path: P,
    ) -> Result<()> {
        unimplemented!()
    }

    async fn rename<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &PMUser,
        from: P,
        to: P,
    ) -> Result<()> {
        unimplemented!()
    }


    async fn rmd<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &PMUser,
        path: P,
    ) -> Result<()> {
        unimplemented!()
    }

    async fn cwd<P: AsRef<Path> + Send + Debug>(
        &self,
        user: &PMUser,
        path: P,
    ) -> Result<()> {
        unimplemented!()
    }
}

impl Metadata for Meta {
    fn len(&self) -> u64 {
        self.inner.len()
    }

    fn is_dir(&self) -> bool {
        self.inner.is_dir()
    }

    fn is_file(&self) -> bool {
        self.inner.is_file()
    }

    fn is_symlink(&self) -> bool {
        self.inner.file_type().is_symlink()
    }

    fn modified(&self) -> Result<SystemTime> {
        self.inner.modified().map_err(|e| e.into())
    }

    fn gid(&self) -> u32 {
        0
    }

    fn uid(&self) -> u32 {
        0
    }
}
