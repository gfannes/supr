set_languages("c++20")
add_rules("mode.release") -- Enable with `xmake f -m release`
add_rules("mode.debug")   -- Enable with `xmake f -m debug`

-- @todo: use gubg via add_requires() or add_packagedirs()
includes("/home/geertf/decode-it/bugb/xmake.lua")

target("supr")
  set_kind("binary")
  add_includedirs("src")
  add_files("src/**.cpp")
  add_deps("gubg")
