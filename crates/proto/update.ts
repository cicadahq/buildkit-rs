import { join } from "https://deno.land/std@0.184.0/path/mod.ts";

async function getLatestVersion(url: string) {
  const res = await fetch(url);
  const data = await res.json();
  const version = data[0].tag_name;
  return version;
}

const latestVersion = await getLatestVersion(
  `https://api.github.com/repos/moby/buildkit/releases`,
);

const githubDir = join("vendor", "github.com");
const buildkitDir = join(githubDir, "moby", "buildkit");

console.log(`Updating buildkit to ${latestVersion}`);

const recursiveUpdateBuildkit = async (dir: string) => {
  for await (const dirEntry of Deno.readDir(dir)) {
    if (dirEntry.isDirectory) {
      await recursiveUpdateBuildkit(join(dir, dirEntry.name));
    } else if (dirEntry.isFile) {
      const filePath = join(dir, dirEntry.name);
      const buildkitPath = filePath.replace(buildkitDir, "");

      const url =
        `https://raw.githubusercontent.com/moby/buildkit/${latestVersion}/${buildkitPath}`;

      const fetchRes = await fetch(url);

      if (!fetchRes.ok) {
        console.log(
          `Failed to update moby/buildkit ${filePath} ${fetchRes.status} ${fetchRes.statusText}`,
        );
        Deno.exit(1);
      }

      console.log(
        `Updating moby/buildkit ${filePath}`,
      );

      await Deno.writeTextFile(filePath, await fetchRes.text());
    }
  }
};

await recursiveUpdateBuildkit(buildkitDir);

const recursiveUpdateOther = async (repo: string, dir: string) => {
  for await (const dirEntry of Deno.readDir(dir)) {
    if (dirEntry.isDirectory) {
      await recursiveUpdateOther(repo, join(dir, dirEntry.name));
    } else if (dirEntry.isFile) {
      const filePath = join(dir, dirEntry.name);
      const path = filePath.replace(join(githubDir, repo), "");

      const url =
        `https://raw.githubusercontent.com/moby/buildkit/${latestVersion}/vendor/github.com/${repo}/${path}`;

      const fetchRes = await fetch(url);

      if (!fetchRes.ok) {
        console.log(
          `Failed to update ${repo} ${filePath} ${fetchRes.status} ${fetchRes.statusText}`,
        );
        Deno.exit(1);
      }

      console.log(
        `Updating ${repo} ${filePath}`,
      );

      await Deno.writeTextFile(filePath, await fetchRes.text());
    }
  }
};

const repos = Array.from(Deno.readDirSync(githubDir)).flatMap((orgEntry) => {
  if (orgEntry.isDirectory) {
    return Array.from(Deno.readDirSync(join(githubDir, orgEntry.name))).map(
      (repoEntry) => {
        if (repoEntry.isDirectory) {
          return join(orgEntry.name, repoEntry.name);
        }
      },
    );
  }
}).filter((repo) => repo !== undefined && repo !== "moby/buildkit");

for (const repo of repos) {
  if (repo) {
    await recursiveUpdateOther(repo, join(githubDir, repo));
  }
}
