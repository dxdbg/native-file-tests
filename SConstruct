#
# Top level SConscript file for native-file-tests
#
rootenv = Environment()

Export('rootenv')

# subdirectories
rootenv.SConscript('#/src/SConscript', variant_dir='#/build/src', duplicate=False)

# default target
rootenv.Default('src')

rootenv.Clean('src', 'build')

# vim: ft=python
