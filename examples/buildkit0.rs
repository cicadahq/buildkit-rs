use std::io::Write;

use buildkit_rs::{llb::exec::Exec, util::system::DEFAULT_PATH_ENV_UNIX};

fn main() {
    let opt = build_opt();
    let bk = buildkit(&opt);
    let out = bk.run(Exec::shlex("ls -l /bin")); // debug output

    let dt: Vec<u8> = out.serialize().unwrap();

    // write to stdout
    std::io::stdout().write_all(&dt).unwrap();
}

fn go_build_base() -> State {
    image("docker.io/library/golang:1.20-alpine")
        .add_env("PATH", format!("/usr/local/go/bin:{DEFAULT_PATH_ENV_UNIX}"))
        .add_env("GOPATH", "/go")
        .run(Exec::shlex("apk add --no-cache g++ linux-headers"))
        .run(Exec::shlex("apk add --no-cache git libseccomp-dev make"))
        .root()
}

fn runc(version: &str) -> State {
    go_build_base()
        .run(Exec::shlex("git clone https://github.com/opencontainers/runc.git /go/src/github.com/opencontainers/runc"))
        .dir("/go/src/github.com/opencontainers/runc")
        .run(Exec::shlex(format!("git checkout -q {version}")))
        .run(Exec::shlex("go build -o /usr/bin/runc ./"))
        .root()
}

fn containerd(version: &str) -> State {
    go_build_base()
        .run(Exec::shlex("apk add --no-cache btrfs-progs-dev"))
        .run(Exec::shlex("git clone https://github.com/containerd/containerd.git /go/src/github.com/containerd/containerd"))
        .dir("/go/src/github.com/containerd/containerd")
        .run(Exec::shlex(format!("git checkout -q {version}")))
        .run(Exec::shlex("make bin/containerd"))
        .root()
}

fn buildkit(opt: &BuildOpt) -> State {
    let src = go_build_base()
        .run(Exec::shlex(
            "git clone https://github.com/moby/buildkit.git /go/src/github.com/moby/buildkit",
        ))
        .dir("/go/src/github.com/moby/buildkit");

    let buildkitd_oci_worker_only = src.run(Exec::shlex(
        "go build -o /bin/buildkitd.oci_only -tags no_containerd_worker ./cmd/buildkitd",
    ));

    let buildkitd = src.run(shlex("go build -o /bin/buildkitd ./cmd/buildkitd"));

    let buildctl = src.run(shlex("go build -o /bin/buildctl ./cmd/buildctl"));

    let mut r = image("docker.io/library/alpine:latest");
    r = copy(buildctl.root(), "/bin/buildctl", r, "/bin/");
    r = copy(runc(&opt.runc), "/usr/bin/runc", r, "/bin/");
    if opt.with_containerd {
        r = copy(
            containerd(&opt.containerd),
            "/go/src/github.com/containerd/containerd/bin/containerd",
            r,
            "/bin/",
        );
        r = copy(buildkitd.root(), "/bin/buildkitd", r, "/bin/");
    } else {
        r = copy(
            buildkitd_oci_worker_only.root(),
            "/bin/buildkitd.oci_only",
            r,
            "/bin/",
        );
    }
    r
}

fn copy(src: State, src_path: &str, dest: State, dest_path: &str) -> State {
    let cp_image = image("docker.io/library/alpine:latest");
    let cp = cp_image.run(shlex(format!("cp -a /src{src_path} /dest{dest_path}",)));
    cp.add_mount("/src", src).add_mount("/dest", dest)
}
