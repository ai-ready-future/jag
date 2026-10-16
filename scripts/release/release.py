#!/usr/bin/env python3
"""The steps of a jag release. Python 3.11 or later, standard library only.

The spec is docs/superpowers/specs/2026-10-14-STORY-48-install-jag-from-github-releases-crates-io-or-npm-design.md.
The names of the archives and the npm packages come from docs/adr/0020-name-the-release-archives-and-the-npm-platform-packages-by.md.

Each command prints its result on standard output. A failed check writes one
line that starts with "release:" on standard error and exits with 1. A line
never holds a member path, a found value, or a secret.
"""

import argparse
import gzip
import hashlib
import io
import json
import os
import re
import shutil
import subprocess
import sys
import tarfile
import tempfile
import tomllib
import urllib.error
import urllib.request
from pathlib import Path

CRATE = "jag-grep"
NPM_SCOPE = "@airf"
NPM_MAIN = f"{NPM_SCOPE}/jag"
REPOSITORY_URL = "git+https://github.com/ai-ready-future/jag.git"
CRATE_REPOSITORY_URL = "https://github.com/ai-ready-future/jag"
LICENSE_ID = "MIT"
NOTICES_NAME = "THIRD-PARTY-LICENSES"
NOTICES_FIRST_LINE = "THIRD-PARTY LICENSES"
CARGO_TOML = Path(__file__).resolve().parents[2] / "Cargo.toml"

# The one table of the platforms (spec, "The platforms").
PLATFORMS = [
    {
        "target": "x86_64-unknown-linux-musl",
        "runner": "ubuntu-24.04",
        "os": "linux",
        "cpu": "x64",
        "label": "Linux x64",
    },
    {
        "target": "aarch64-unknown-linux-musl",
        "runner": "ubuntu-24.04-arm",
        "os": "linux",
        "cpu": "arm64",
        "label": "Linux arm64",
    },
    {
        "target": "aarch64-apple-darwin",
        "runner": "macos-15",
        "os": "darwin",
        "cpu": "arm64",
        "label": "macOS arm64",
    },
    {
        "target": "x86_64-apple-darwin",
        "runner": "macos-15-intel",
        "os": "darwin",
        "cpu": "x64",
        "label": "macOS x64",
    },
]

VERSION_RE = re.compile(r"^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$")
INSTALL_SCRIPTS = ("preinstall", "install", "postinstall", "prepare")
CRATE_FILES = {
    "Cargo.toml",
    "Cargo.toml.orig",
    "Cargo.lock",
    "README.md",
    "LICENSE",
    ".cargo_vcs_info.json",
}
CRATE_REQUIRED = {"Cargo.toml", "Cargo.lock", "README.md", "LICENSE"}
NPM_REGISTRY = "https://registry.npmjs.org/"
USER_AGENT = "jag-release (https://github.com/ai-ready-future/jag)"


class ReleaseError(Exception):
    """A failed check. The message is one line and holds no private data."""


def fail(message):
    raise ReleaseError(message)


def platform_by_target(target):
    for platform in PLATFORMS:
        if platform["target"] == target:
            return platform
    fail(f"the target {target} is not one of the four targets.")


def npm_platform_name(platform):
    return f"{NPM_SCOPE}/jag-{platform['os']}-{platform['cpu']}"


def npm_tarball_name(name, version):
    """The file name that npm pack writes for a scoped package."""
    return f"{name.lstrip('@').replace('/', '-')}-{version}.tgz"


def archive_name(version, target):
    return f"jag-{version}-{target}.tar.gz"


def check_version_form(version):
    if not VERSION_RE.match(version):
        fail(f"the version {version} is not of the form MAJOR.MINOR.PATCH.")


def read_json(path):
    with open(path, encoding="utf-8") as handle:
        return json.load(handle)


def write_json(path, data):
    Path(path).write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")


# ---------------------------------------------------------------- version


def cargo_version(cargo_toml, cargo_lock=None):
    """The version of Cargo.toml. It must agree with Cargo.lock when that file exists."""
    with open(cargo_toml, "rb") as handle:
        version = tomllib.load(handle)["package"]["version"]
    check_version_form(version)
    if cargo_lock and Path(cargo_lock).is_file():
        with open(cargo_lock, "rb") as handle:
            lock = tomllib.load(handle)
        for package in lock.get("package", []):
            if package.get("name") == CRATE and package.get("version") != version:
                fail(
                    f"the version {package.get('version')} in Cargo.lock does not match "
                    f"the version {version} in Cargo.toml."
                )
    return version


def command_version(args):
    version = cargo_version(args.cargo_toml, args.cargo_lock)
    if args.tag is not None and args.tag != f"v{version}":
        fail(f"the tag {args.tag} does not match the version {version} in Cargo.toml.")
    print(version)


def command_matrix(args):
    include = [
        {
            "target": p["target"],
            "runner": p["runner"],
            "os": p["os"],
            "cpu": p["cpu"],
            "package": npm_platform_name(p),
        }
        for p in PLATFORMS
    ]
    print(json.dumps({"include": include}, separators=(",", ":")))


# ----------------------------------------------------------- binary check


def parse_readelf_headers(text):
    """True when the output of readelf -lW shows an INTERP header."""
    return any(line.split()[:1] == ["INTERP"] for line in text.splitlines())


def parse_readelf_dynamic(text):
    """True when the output of readelf -dW shows a (NEEDED) entry."""
    return any("(NEEDED)" in line for line in text.splitlines())


def parse_otool_libraries(text):
    """The library paths that otool -L shows. The first line names the file."""
    paths = []
    for line in text.splitlines()[1:]:
        line = line.strip()
        if line:
            paths.append(line.split(" (", 1)[0])
    return paths


def bad_macos_libraries(paths):
    return [
        p
        for p in paths
        if not (p.startswith("/usr/lib/") or p.startswith("/System/Library/Frameworks/"))
    ]


def run_text(cmd):
    result = subprocess.run(cmd, capture_output=True, text=True, check=False)
    if result.returncode != 0:
        fail(f"the command {cmd[0]} failed with exit {result.returncode}.")
    return result.stdout


def command_check_binary(args):
    platform = platform_by_target(args.target)
    binary = str(Path(args.binary).resolve())
    if platform["os"] == "linux":
        if parse_readelf_headers(run_text(["readelf", "-lW", binary])):
            fail("the binary has an INTERP header. It is not static.")
        if parse_readelf_dynamic(run_text(["readelf", "-dW", binary])):
            fail("the binary has a NEEDED entry. It is not static.")
    else:
        bad = bad_macos_libraries(parse_otool_libraries(run_text(["otool", "-L", binary])))
        if bad:
            fail(f"the binary uses {len(bad)} library outside /usr/lib and the system frameworks.")
    result = subprocess.run([binary, "--version"], capture_output=True, text=True, env={}, check=False)
    if result.returncode != 0 or result.stdout.strip() != f"jag {args.version}":
        fail(f"the binary does not print jag {args.version} with an empty environment.")
    print(f"release: the binary for {args.target} passes.")


# --------------------------------------------------------------- archive


def commit_time():
    result = subprocess.run(["git", "log", "-1", "--format=%ct"], capture_output=True, text=True, check=False)
    if result.returncode != 0:
        fail("cannot read the time of the commit.")
    return int(result.stdout.strip())


def make_archive(version, target, binary, readme, license_file, notices, out_dir, mtime):
    """Write the archive and its checksum file. The same input gives the same archive."""
    out_dir = Path(out_dir)
    out_dir.mkdir(parents=True, exist_ok=True)
    folder = f"jag-{version}-{target}"
    name = archive_name(version, target)
    path = out_dir / name
    entries = [
        (f"{folder}/", None, 0o755),
        (f"{folder}/jag", binary, 0o755),
        (f"{folder}/README.md", readme, 0o644),
        (f"{folder}/LICENSE", license_file, 0o644),
        (f"{folder}/{NOTICES_NAME}", notices, 0o644),
    ]
    with open(path, "wb") as raw:
        with gzip.GzipFile(filename="", mode="wb", fileobj=raw, mtime=0, compresslevel=9) as gz:
            with tarfile.open(fileobj=gz, mode="w", format=tarfile.PAX_FORMAT) as tar:
                for member_name, source, mode in entries:
                    info = tarfile.TarInfo(member_name.rstrip("/"))
                    info.mtime = mtime
                    info.mode = mode
                    info.uid = info.gid = 0
                    info.uname = info.gname = ""
                    if source is None:
                        info.type = tarfile.DIRTYPE
                        tar.addfile(info)
                    else:
                        data = Path(source).read_bytes()
                        info.size = len(data)
                        tar.addfile(info, io.BytesIO(data))
    digest = hashlib.sha256(path.read_bytes()).hexdigest()
    checksum = out_dir / f"{name}.sha256"
    checksum.write_text(f"{digest}  {name}\n", encoding="utf-8")
    return path, checksum


def command_archive(args):
    platform_by_target(args.target)
    check_version_form(args.version)
    mtime = args.mtime if args.mtime is not None else commit_time()
    path, _ = make_archive(
        args.version, args.target, args.binary, args.readme, args.license, args.notices, args.out, mtime
    )
    print(path.name)


# ------------------------------------------------------------------ npm


def npm_main_package(template, version):
    package = dict(template)
    package["version"] = version
    package["optionalDependencies"] = {npm_platform_name(p): version for p in PLATFORMS}
    package["optionalDependencies"] = dict(sorted(package["optionalDependencies"].items()))
    return package


def command_npm_main(args):
    check_version_form(args.version)
    source = Path(args.source)
    out = Path(args.out) / "jag"
    if out.exists():
        shutil.rmtree(out)
    (out / "bin").mkdir(parents=True)
    write_json(out / "package.json", npm_main_package(read_json(source / "package.json"), args.version))
    shutil.copyfile(source / "bin" / "jag.js", out / "bin" / "jag.js")
    os.chmod(out / "bin" / "jag.js", 0o755)
    shutil.copyfile(args.readme, out / "README.md")
    shutil.copyfile(args.license, out / "LICENSE")
    print(out)


def npm_platform_description(platform):
    return f"The jag binary for {platform['label']}. Install {NPM_MAIN}, not this package."


def npm_platform_package(platform, version):
    return {
        "name": npm_platform_name(platform),
        "version": version,
        "description": npm_platform_description(platform),
        "license": LICENSE_ID,
        "repository": {"type": "git", "url": REPOSITORY_URL},
        "homepage": "https://github.com/ai-ready-future/jag#readme",
        "os": [platform["os"]],
        "cpu": [platform["cpu"]],
        "files": ["bin/jag", NOTICES_NAME],
        "publishConfig": {"access": "public"},
    }


def platform_readme(template_text, platform):
    return template_text.replace("{package}", npm_platform_name(platform)).replace("{platform}", platform["label"])


def command_npm_platform(args):
    platform = platform_by_target(args.target)
    check_version_form(args.version)
    out = Path(args.out) / f"jag-{platform['os']}-{platform['cpu']}"
    if out.exists():
        shutil.rmtree(out)
    (out / "bin").mkdir(parents=True)
    write_json(out / "package.json", npm_platform_package(platform, args.version))
    shutil.copyfile(args.binary, out / "bin" / "jag")
    os.chmod(out / "bin" / "jag", 0o755)
    template = Path(args.readme).read_text(encoding="utf-8")
    (out / "README.md").write_text(platform_readme(template, platform), encoding="utf-8")
    shutil.copyfile(args.license, out / "LICENSE")
    shutil.copyfile(args.notices, out / NOTICES_NAME)
    print(out)


def read_npm_tarball(path):
    """The members of an npm tarball: {name: (mode, bytes)}."""
    members = {}
    with tarfile.open(path, "r:gz") as tar:
        for info in tar:
            if info.isfile():
                members[info.name] = (info.mode & 0o777, tar.extractfile(info).read())
    return members


def cargo_metadata(cargo_toml=CARGO_TOML):
    """The license, repository, and description of a Cargo.toml (a path, or the text of the file)."""
    if isinstance(cargo_toml, bytes):
        data = tomllib.loads(cargo_toml.decode("utf-8"))
    else:
        with open(cargo_toml, "rb") as handle:
            data = tomllib.load(handle)
    package = data.get("package") or {}
    return {key: package.get(key) for key in ("license", "repository", "description")}


def check_npm_package_json(package, version):
    """The checks of one package.json. Return the kind: main or platform."""
    name = package.get("name")
    main_name = NPM_MAIN
    platforms = {npm_platform_name(p): p for p in PLATFORMS}
    if name != main_name and name not in platforms:
        fail("the package name is not one of the five names.")
    if package.get("version") != version:
        fail(f"the version of the package is not {version}.")
    scripts = package.get("scripts") or {}
    for script in INSTALL_SCRIPTS:
        if script in scripts:
            fail(f"the package has a {script} script.")
    if (package.get("repository") or {}).get("url") != REPOSITORY_URL:
        fail("the repository url of the package is not the url of the repository.")
    if package.get("license") != LICENSE_ID:
        fail(f"the license of the package is not {LICENSE_ID}.")
    if (package.get("publishConfig") or {}).get("access") != "public":
        fail("the package is not public.")
    if name == main_name:
        if package.get("description") != cargo_metadata()["description"]:
            fail("the description of the main package is not the description in Cargo.toml.")
        if package.get("bin") != {"jag": "bin/jag.js"}:
            fail("the main package has no bin entry for jag.")
        if (package.get("engines") or {}).get("node") != ">=18":
            fail("the main package does not need Node.js 18 or later.")
        expected = {npm_platform_name(p): version for p in PLATFORMS}
        if package.get("optionalDependencies") != expected:
            fail("the optional dependencies of the main package are not the four exact versions.")
        return "main"
    platform = platforms[name]
    if package.get("description") != npm_platform_description(platform):
        fail("the description of the platform package is not the expected text.")
    if package.get("os")!= [platform["os"]] or package.get("cpu") != [platform["cpu"]]:
        fail("the platform package has not exactly one os value and one cpu value of its platform.")
    if "libc" in package:
        fail("the platform package has a libc value.")
    if "bin" in package:
        fail("the platform package has a bin field.")
    return "platform"


def command_check_npm_tarball(args):
    members = read_npm_tarball(args.file)
    if "package/package.json" not in members:
        fail("the tarball has no package.json.")
    package = json.loads(members["package/package.json"][1])
    kind = check_npm_package_json(package, args.version)
    binary = "package/bin/jag.js" if kind == "main" else "package/bin/jag"
    expected = {"package/package.json", "package/README.md", "package/LICENSE", binary}
    if kind == "platform":
        expected.add(f"package/{NOTICES_NAME}")
    if set(members) != expected:
        fail("the list of files of the tarball is not the expected list.")
    if kind == "platform" and members[binary][0] != 0o755:
        fail("the binary in the platform package has not the mode 0755.")
    print(f"release: the tarball of {package['name']} passes.")


NPM_DEPENDENCY_FIELDS = ("dependencies", "peerDependencies", "bundleDependencies", "bundledDependencies")


def check_npm_deps(package):
    """The npm package has no third-party dependency (FEATURE-22 spec, decision 13)."""
    for field in NPM_DEPENDENCY_FIELDS:
        if field in package:
            fail(f"the package has a {field} field.")
    allowed = {npm_platform_name(p) for p in PLATFORMS}
    for name in package.get("optionalDependencies") or {}:
        if name not in allowed:
            fail(f"the optional dependency {name} is not a platform package.")


def command_check_npm_deps(args):
    check_npm_deps(read_json(args.file))
    print(f"release: {args.file} has no third-party dependency.")


def extract_npm_tarball(path, destination):
    """Unpack a tarball as npm does: strip the first folder, keep the mode."""
    destination = Path(destination)
    with tarfile.open(path, "r:gz") as tar:
        for info in tar:
            if not info.isfile():
                continue
            parts = Path(info.name).parts
            if len(parts) < 2 or parts[0] != "package" or ".." in parts:
                fail("the tarball has a member outside the folder package.")
            target = destination.joinpath(*parts[1:])
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(tar.extractfile(info).read())
            os.chmod(target, info.mode & 0o777)


def node_executable():
    node = shutil.which("node")
    if not node:
        fail("node is not installed.")
    return node


def command_local_install(args):
    node = node_executable()
    platform_package = json.loads(read_npm_tarball(args.platform)["package/package.json"][1])
    with tempfile.TemporaryDirectory() as tmp:
        modules = Path(tmp) / "node_modules" / "@airf"
        extract_npm_tarball(args.main, modules / "jag")
        extract_npm_tarball(args.platform, modules / platform_package["name"].split("/", 1)[1])
        launcher = modules / "jag" / "bin" / "jag.js"
        run = subprocess.run([node, str(launcher), "--version"], capture_output=True, text=True, env={}, check=False)
        if run.returncode != 0 or run.stdout.strip() != f"jag {args.version}":
            fail(f"the launcher does not print jag {args.version}.")
        run = subprocess.run(
            [node, str(launcher), "--no-such-option"], capture_output=True, text=True, env={}, check=False
        )
        if run.returncode != 2:
            fail(f"the launcher gave exit {run.returncode} for a bad option, not 2.")
    print("release: the launcher and the binary run as npm installs them.")


def parse_semver_prefix(text):
    match = re.match(r"^(\d+)\.(\d+)\.(\d+)", text.strip())
    if not match:
        fail("the version text has no MAJOR.MINOR.PATCH.")
    return tuple(int(part) for part in match.groups())


def command_check_npm_version(args):
    if parse_semver_prefix(args.version) < (11, 5, 1):
        fail("trusted publishing needs npm 11.5.1 or later.")
    print("release: the npm version is enough.")


def check_install_log(text):
    """Each `http fetch` line names the npm registry, and the log has 2 such lines or more."""
    count = 0
    for line in text.splitlines():
        if "http fetch" not in line:
            continue
        urls = re.findall(r"https?://\S+", line)
        if not urls or not all(url.startswith(NPM_REGISTRY) for url in urls):
            fail("an http fetch line of the npm log names a url outside the npm registry.")
        count += 1
    if count < 2:
        fail("the npm log has fewer than 2 http fetch lines.")
    return count


def command_check_npm_install(args):
    platform = platform_by_target(args.target)
    check_install_log(Path(args.log).read_text(encoding="utf-8", errors="replace"))
    scope = Path(args.dir) / "node_modules" / "@airf"
    found = sorted(entry.name for entry in scope.iterdir()) if scope.is_dir() else []
    expected = sorted(["jag", f"jag-{platform['os']}-{platform['cpu']}"])
    if found != expected:
        fail("node_modules/@airf does not hold only jag and the package of this platform.")
    run = subprocess.run(
        [str(Path(args.dir) / "node_modules" / ".bin" / "jag"), "--version"],
        capture_output=True,
        text=True,
        check=False,
    )
    if run.returncode != 0 or run.stdout.strip() != f"jag {args.version}":
        fail(f"the installed jag does not print jag {args.version}.")
    print("release: the npm install passes.")


# ------------------------------------------------------------- the crate


def check_crate_list(lines):
    """Each path of cargo package --list is under src/ or in the short list. Return nothing, or fail."""
    seen = set()
    for number, line in enumerate(lines, start=1):
        path = line.rstrip("\n")
        if not path:
            continue
        seen.add(path)
        if path.startswith("src/") or path in CRATE_FILES:
            continue
        fail(f"line {number} of the crate list is not an allowed path.")
    if not CRATE_REQUIRED <= seen:
        fail("the crate list lacks Cargo.toml, Cargo.lock, README.md, or LICENSE.")


def command_check_crate_list(args):
    if args.file == "-":
        lines = sys.stdin.read().splitlines()
    else:
        lines = Path(args.file).read_text(encoding="utf-8").splitlines()
    check_crate_list(lines)
    print("release: the crate list passes.")


def check_crate_metadata(crate_file):
    """The license, repository, and description of the Cargo.toml in a packed .crate file."""
    manifests = []
    with tarfile.open(crate_file, "r:gz") as tar:
        for info in tar:
            parts = Path(info.name).parts
            if info.isfile() and len(parts) == 2 and parts[1] == "Cargo.toml":
                manifests.append(tar.extractfile(info).read())
    if len(manifests) != 1:
        fail("the crate file does not hold exactly one Cargo.toml in its folder.")
    found = cargo_metadata(manifests[0])
    expected = cargo_metadata()
    if found["license"] != LICENSE_ID:
        fail(f"the license of the crate is not {LICENSE_ID}.")
    if found["repository"] != CRATE_REPOSITORY_URL:
        fail("the repository of the crate is not the url of the repository.")
    if not expected["description"] or found["description"] != expected["description"]:
        fail("the description of the crate is not the description in Cargo.toml of the repository.")


def command_check_crate_metadata(args):
    check_crate_metadata(args.file)
    print("release: the metadata of the crate passes.")


# ------------------------------------------------------------- the notices


def check_notices(path):
    """The notice file is UTF-8, is not empty, and starts with the fixed first line."""
    data = Path(path).read_bytes()
    try:
        text = data.decode("utf-8")
    except UnicodeDecodeError:
        fail(f"the file {NOTICES_NAME} is not UTF-8.")
    if not text.strip():
        fail(f"the file {NOTICES_NAME} is empty.")
    if text.split("\n", 1)[0] != NOTICES_FIRST_LINE:
        fail(f"the first line of the file {NOTICES_NAME} is not {NOTICES_FIRST_LINE}.")


def command_check_notices(args):
    check_notices(args.file)
    print(f"release: the file {NOTICES_NAME} passes.")


# ------------------------------------------------------------- the scan

PRINTABLE_RUN = re.compile(rb"[\x20-\x7e\t]{4,}")
BINARY_PROBE = 8192


def decode(data):
    return data.decode("utf-8", errors="replace")


def split_lines(text):
    """The lines of a text. Only a newline ends a line. A final newline gives no empty line."""
    pieces = text.split("\n")
    if pieces and pieces[-1] == "":
        pieces.pop()
    return pieces


class TextWriter:
    """The text file and the index file of the text mode of check.sh. Line N of one names line N of the other."""

    def __init__(self):
        self.text = []
        self.index = []

    def add(self, text, label):
        for piece in split_lines(text) if text else []:
            self.text.append(piece)
            self.index.append(label)

    def add_name(self, text, label):
        # A name with a newline gives one line for each piece. All pieces share the label.
        for piece in text.split("\n"):
            self.text.append(piece)
            self.index.append(label)


def texts_for_member(writer, file_label, number, data):
    if data is None:
        return
    prefix = f"{file_label} member {number}"
    if b"\x00" in data[:BINARY_PROBE]:
        for found in PRINTABLE_RUN.finditer(data):
            writer.text.append(found.group().decode("ascii"))
            writer.index.append(f"{prefix} byte {found.start()}")
    else:
        for line_number, piece in enumerate(split_lines(decode(data)), start=1):
            writer.text.append(piece)
            writer.index.append(f"{prefix} line {line_number}")


def texts_for_file(writer, path):
    path = Path(path)
    label = path.name
    if path.name.endswith(".sha256"):
        for line_number, piece in enumerate(split_lines(decode(path.read_bytes())), start=1):
            writer.text.append(piece)
            writer.index.append(f"{label} line {line_number}")
        return
    with tarfile.open(path, "r:gz") as tar:
        for number, info in enumerate(tar, start=1):
            writer.add_name(info.name, f"{label} member {number} name")
            if info.issym() or info.islnk():
                writer.add_name(info.linkname, f"{label} member {number} link")
            data = tar.extractfile(info).read() if info.isfile() else None
            texts_for_member(writer, label, number, data)


def command_texts(args):
    writer = TextWriter()
    for path in args.files:
        texts_for_file(writer, path)
    Path(args.text).parent.mkdir(parents=True, exist_ok=True)
    Path(args.index).parent.mkdir(parents=True, exist_ok=True)
    Path(args.text).write_text("".join(f"{line}\n" for line in writer.text), encoding="utf-8")
    Path(args.index).write_text("".join(f"{label}\n" for label in writer.index), encoding="utf-8")
    print(f"release: {len(writer.text)} lines to scan from {len(args.files)} files.")


# ----------------------------------------------------- the GitHub Release


def expected_release_files(version):
    names = []
    for platform in PLATFORMS:
        archive = archive_name(version, platform["target"])
        names += [archive, f"{archive}.sha256"]
    return sorted(names)


def command_check_release(args):
    directory = Path(args.dir)
    check_version_form(args.version)
    found = sorted(entry.name for entry in directory.iterdir())
    if found != expected_release_files(args.version):
        fail("the release does not have exactly the 4 archives and the 4 checksum files.")
    for platform in PLATFORMS:
        name = archive_name(args.version, platform["target"])
        digest = hashlib.sha256((directory / name).read_bytes()).hexdigest()
        line = (directory / f"{name}.sha256").read_text(encoding="utf-8")
        if line != f"{digest}  {name}\n":
            fail(f"the checksum file of the archive for {platform['target']} does not agree.")
    print("release: the release files pass.")


# ------------------------------------------------------------- registries


def registry_url(registry, name, version):
    if registry == "crates":
        return f"https://crates.io/api/v1/crates/{name}/{version}"
    return f"{NPM_REGISTRY}{name.replace('/', '%2f')}/{version}"


def version_exists(registry, name, version, opener=urllib.request.urlopen):
    """True when the registry has the version. False on 404. Any other answer is an error."""
    request = urllib.request.Request(registry_url(registry, name, version), headers={"User-Agent": USER_AGENT})
    try:
        with opener(request, timeout=30) as response:
            return response.status == 200
    except urllib.error.HTTPError as error:
        if error.code == 404:
            return False
        fail(f"the registry {registry} answered with HTTP {error.code}.")
    except urllib.error.URLError:
        fail(f"cannot reach the registry {registry}.")


def command_exists(args):
    check_version_form(args.version)
    print("true" if version_exists(args.registry, args.name, args.version) else "false")


# ------------------------------------------------------------------ main


def build_parser():
    parser = argparse.ArgumentParser(description="The steps of a jag release.")
    sub = parser.add_subparsers(dest="command", required=True)

    p = sub.add_parser("version", help="print the version of Cargo.toml")
    p.add_argument("--cargo-toml", default="Cargo.toml")
    p.add_argument("--cargo-lock", default="Cargo.lock")
    p.add_argument("--tag")
    p.set_defaults(func=command_version)

    p = sub.add_parser("matrix", help="print the build matrix as JSON")
    p.set_defaults(func=command_matrix)

    p = sub.add_parser("check-binary")
    p.add_argument("--target", required=True)
    p.add_argument("--version", required=True)
    p.add_argument("binary")
    p.set_defaults(func=command_check_binary)

    p = sub.add_parser("archive")
    p.add_argument("--target", required=True)
    p.add_argument("--version", required=True)
    p.add_argument("--binary", required=True)
    p.add_argument("--out", required=True)
    p.add_argument("--readme", default="README.md")
    p.add_argument("--license", default="LICENSE")
    p.add_argument("--notices", required=True, help=f"the file {NOTICES_NAME}")
    p.add_argument("--mtime", type=int, help="default: the time of the commit")
    p.set_defaults(func=command_archive)

    p = sub.add_parser("npm-main")
    p.add_argument("--version", required=True)
    p.add_argument("--out", required=True)
    p.add_argument("--source", default="npm/jag")
    p.add_argument("--readme", default="README.md")
    p.add_argument("--license", default="LICENSE")
    p.set_defaults(func=command_npm_main)

    p = sub.add_parser("npm-platform")
    p.add_argument("--target", required=True)
    p.add_argument("--version", required=True)
    p.add_argument("--binary", required=True)
    p.add_argument("--out", required=True)
    p.add_argument("--readme", default="npm/platform/README.md")
    p.add_argument("--license", default="LICENSE")
    p.add_argument("--notices", required=True, help=f"the file {NOTICES_NAME}")
    p.set_defaults(func=command_npm_platform)

    p = sub.add_parser("check-npm-tarball")
    p.add_argument("--version", required=True)
    p.add_argument("file")
    p.set_defaults(func=command_check_npm_tarball)

    p = sub.add_parser("check-npm-deps")
    p.add_argument("file", help="a package.json")
    p.set_defaults(func=command_check_npm_deps)

    p = sub.add_parser("local-install")
    p.add_argument("--main", required=True)
    p.add_argument("--platform", required=True)
    p.add_argument("--version", required=True)
    p.set_defaults(func=command_local_install)

    p = sub.add_parser("check-crate-list")
    p.add_argument("file", help="a file, or - for standard input")
    p.set_defaults(func=command_check_crate_list)

    p = sub.add_parser("check-crate-metadata")
    p.add_argument("file", help="a .crate file")
    p.set_defaults(func=command_check_crate_metadata)

    p = sub.add_parser("check-notices")
    p.add_argument("file", help=f"the file {NOTICES_NAME}")
    p.set_defaults(func=command_check_notices)

    p = sub.add_parser("texts")
    p.add_argument("--text", required=True)
    p.add_argument("--index", required=True)
    p.add_argument("files", nargs="+")
    p.set_defaults(func=command_texts)

    p = sub.add_parser("check-release")
    p.add_argument("--dir", required=True)
    p.add_argument("--version", required=True)
    p.set_defaults(func=command_check_release)

    p = sub.add_parser("exists")
    p.add_argument("--registry", choices=["crates", "npm"], required=True)
    p.add_argument("--name", required=True)
    p.add_argument("--version", required=True)
    p.set_defaults(func=command_exists)

    p = sub.add_parser("check-npm-version")
    p.add_argument("version", help="the output of npm --version")
    p.set_defaults(func=command_check_npm_version)

    p = sub.add_parser("check-npm-install")
    p.add_argument("--log", required=True)
    p.add_argument("--dir", required=True)
    p.add_argument("--target", required=True)
    p.add_argument("--version", required=True)
    p.set_defaults(func=command_check_npm_install)

    return parser


def main(argv=None):
    args = build_parser().parse_args(argv)
    try:
        args.func(args)
    except ReleaseError as error:
        print(f"release: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
