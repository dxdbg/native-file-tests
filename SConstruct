#
# Top level SConscript file for native-file-tests
#
import platform

rootenv = Environment()

Export('rootenv')

# subdirectories
rootenv.SConscript('#/src/SConscript', variant_dir='#/build/' + platform.platform(), duplicate=False)

# default target
rootenv.Default('src')

rootenv.Clean('src', 'build')

# vim: ft=python
