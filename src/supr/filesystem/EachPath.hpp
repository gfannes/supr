#ifndef HEADER_supr_filesystem_EachPath_hpp_ALREAD_INCLUDED
#define HEADER_supr_filesystem_EachPath_hpp_ALREAD_INCLUDED

#include <gubg/mss.hpp>

#include <filesystem>

namespace supr::filesystem {

    class EachPath
    {
    public:
        struct Config
        {
            bool hidden = false;
        };

        EachPath(const Config &config)
            : config_(config) {}

        template<typename Ftor>
        void operator()(const std::filesystem::path &path, Ftor &&ftor)
        {
            for (const auto &entry : std::filesystem::directory_iterator{path})
            {
                const auto path = entry.path();

                if (!config_.hidden)
                {
                    const char *cstr = path.filename().c_str();
                    if (cstr[0] == '.' && cstr[1] != '.')
                        continue;
                }

                if (std::filesystem::is_regular_file(path))
                {
                    ftor(path);
                }
                else if (std::filesystem::is_directory(path))
                {
                    operator()(path, ftor);
                }
                else
                {
                    std::cout << "??" << path << std::endl;
                }
            }
        }

    private:
        Config config_;
    };

} // namespace supr::file

#endif
