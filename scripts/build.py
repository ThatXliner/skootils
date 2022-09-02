#!/usr/bin/env python
import glob
import platform
import re
import shutil
import subprocess
import sys
import time
from os.path import getmtime
from pathlib import Path
from typing import NamedTuple

EXTENSION = ".exe" if platform.system() == "Windows" else ""

TARGET_TRIPLE = re.search(
    r"host: (\S+)",
    # skipcq
    subprocess.run(["rustc", "-vV"], capture_output=True, text=True).stdout,
).group(1)


def get_new_name(binary: Path) -> Path:
    return binary.with_name(f"{binary.stem}-{TARGET_TRIPLE}{EXTENSION}")


def rename(binary: Path):
    binary.replace(get_new_name(binary))


class Job(NamedTuple):
    proc: subprocess.Popen
    output: Path


def is_verbose():
    return "--verbose" in sys.argv


def start_build(spec: Path) -> subprocess.Popen:
    output = subprocess.Popen(
        ["pyinstaller", str(spec.name)],
        cwd=str(spec.parent),
        stdout=None if is_verbose() else subprocess.DEVNULL,
        stderr=None if is_verbose() else subprocess.DEVNULL,
    )
    if "--serial" in sys.argv:
        output.wait()
    return output


specfiles = map(
    Path, glob.glob("src/learnatvcs/*.spec") + glob.glob("src/powerschool/*.spec")
)
targets = {
    prereq.parent / "dist" / prereq.stem: [prereq]
    + list((prereq.parent / prereq.parent.stem).glob("**/*.py"))
    for prereq in specfiles
}
if is_verbose():
    print("Finished building target graph")
jobs = set()
for target, prerequisites in targets.items():
    expected_output = get_new_name(target)
    if (
        "--force" in sys.argv
        or not expected_output.exists()
        or any(
            map(
                # "Cell variable defined in loop"
                # shouldn't be a problem because
                # we're using the `any` function
                # to use it immediantly after
                lambda mod_time: mod_time >= getmtime(expected_output),
                map(getmtime, prerequisites),
            )
        )
    ):
        if is_verbose():
            print(f"{target} needs to be built")
        jobs.add(Job(start_build(prerequisites[0]), target))
        print("Sent", prerequisites[0], "to build worker")
if not jobs:
    print("Nothing needs to be built")
    sys.exit(0)
print(f"Building {len(jobs)} binaries")
start = time.time()
while jobs:
    for job in list(jobs):
        status = job.proc.poll()
        if status == None:
            continue
        jobs.remove(job)
        if status == 0:
            rename(job.output.with_suffix(EXTENSION))
            print("Finished building", job.output)
        else:
            print(f"ERROR: {job.output} failed to build")
            sys.exit(1)
    time.sleep(0.5)
print(f"Finished in {time.time() - start:0.2f} seconds (binaries unverified)")
