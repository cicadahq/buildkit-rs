use pin_project::pin_project;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt, ReadBuf, Result};
use tokio::process::{Child, ChildStdin, ChildStdout};

#[pin_project]
pub(crate) struct BuildkitStdio {
    child: Child,
    #[pin]
    stdin: ChildStdin,
    #[pin]
    stdout: ChildStdout,
}

impl BuildkitStdio {
    /// Create a new BuildkitStdio from a container name.
    ///
    /// ## Panics
    ///
    /// This function will panic if [child] does not have a stdin or stdout.
    pub fn new(mut child: Child) -> BuildkitStdio {
        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();

        Self {
            child,
            stdin,
            stdout,
        }
    }

    async fn kill(&mut self) -> Result<()> {
        self.stdin.shutdown().await?;
        self.child.kill().await?;
        Ok(())
    }
}

impl AsyncRead for BuildkitStdio {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<()>> {
        self.project().stdout.poll_read(cx, buf)
    }
}

impl AsyncWrite for BuildkitStdio {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize>> {
        self.project().stdin.poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        self.project().stdin.poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        self.project().stdin.poll_shutdown(cx)
    }

    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[std::io::IoSlice<'_>],
    ) -> Poll<std::result::Result<usize, std::io::Error>> {
        self.project().stdin.poll_write_vectored(cx, bufs)
    }

    fn is_write_vectored(&self) -> bool {
        self.stdin.is_write_vectored()
    }
}
