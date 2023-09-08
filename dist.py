#!/usr/bin/python
#
# Script to package a .zip containing the artifacts
#
# NOTE: this is implemented in Python to allow it to be used on Windows
# without requiring Cygwin or similar.
#

import sys
import shutil
import os
import glob
import hashlib

if len(sys.argv) != 3:
    print("Usage: script <platform name> <version>")
    sys.exit(1)

platform_name = sys.argv[1]
version = sys.argv[2]

platform_dir = "native-file-tests-{}-{}".format(platform_name, version)
os.mkdir(platform_dir)

for path in glob.glob("build/*/*"):
    with open(path) as f:
        base = os.path.basename(path)
        data = f.read()
        sha = hashlib.sha256(data).hexdigest()
        shutil.copy(path, "{}/{}.{}".format(platform_dir, base, sha))

shutil.make_archive(platform_dir, "zip", ".", platform_dir)
