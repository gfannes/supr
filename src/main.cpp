#include <supr/Options.hpp>

#include <filesystem>
#include <iostream>

namespace gubg {

class EachFile {
public:
  struct Config {
    bool hidden = false;
  };

  EachFile(const Config &config) : config_(config) {}

  template <typename Ftor>
  void operator()(const std::filesystem::path &path, Ftor &&ftor) {
    for (const auto &entry : std::filesystem::directory_iterator{path}) {
      const auto path = entry.path();

      if (!config_.hidden) {
        const char *cstr = path.filename().c_str();
        if (cstr[0] == '.' && cstr[1] != '.')
          continue;
      }

      if (std::filesystem::is_regular_file(path)) {
        ftor(path);
      } else if (std::filesystem::is_directory(path)) {
        operator()(path, ftor);
      } else {
        std::cout << "??" << path << std::endl;
      }
    }
  }

private:
  Config config_;
};

} // namespace gubg

int main(int argc, const char **argv) {

  const char *folder = argc >= 2 ? argv[1] : "/home/geertf/supr";

  std::uintmax_t total_size{};

  gubg::EachFile each_file({});
  each_file(folder, [&](const std::filesystem::path &path) {
    const auto size = std::filesystem::file_size(path);
    total_size += size;
    std::cout << path << ';' << size << std::endl;
  });
  std::cout << "TOTAL SIZE;" << total_size << std::endl;
  return 0;
}
