set_languages("c++20")
add_rules("mode.release") -- Enable with `xmake f -m release`
add_rules("mode.debug")   -- Enable with `xmake f -m debug`

target("supr")
  set_kind("binary")
  add_includedirs("src")
  add_files("src/**.cpp")
