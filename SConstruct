#
# Top level SConscript file for native-file-tests
#
import platform
import os

variables = Variables([".nft.config"])
variables.Add("MSVC_USE_SCRIPT", "MSVC_USE_SCRIPT value to pass to the SCons environment")
variables.Add("MSVC_VERSION", "MSVC_VERSION value to pass to the SCons environment")

rootenv = Environment(variables = variables)

if "configure" in COMMAND_LINE_TARGETS:
    variables.Save(".nft.config", rootenv)
    Exit(0)

# subdirectories
rootenv.SConscript('#/src/SConscript', variant_dir='#/build/' + platform.platform(), duplicate=False, exports='rootenv')

# default target
rootenv.Default('src')

rootenv.Clean('src', 'build')

# vim: ft=python
