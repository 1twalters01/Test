project('meson_with_lua', 'c',
  version : '0.1',
  default_options : ['warning_level=3'])

gtkdep = dependency('gtk4')
luadep = dependency('lua5.4')

exe = executable('../target/meson_with_lua', 'src/meson_with_lua.c',
  install : true, dependencies : [gtkdep, luadep])

test('basic', exe)
