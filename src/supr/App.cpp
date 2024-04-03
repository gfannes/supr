#include <supr/App.hpp>

#include <supr/filesystem/EachPath.hpp>

#include <gubg/mss.hpp>

#include <iostream>

namespace supr {

    bool App::run()
    {
        MSS_BEGIN(bool);

        MSS(options_.verb);
        switch (*options_.verb)
        {
            case Verb::PrintHelp: print_help_(); break;
            case Verb::Info: MSS(collect_info_()); break;

            default:
                MSS(false, std::cout << "Unknown verb " << *options_.verb << std::endl);
                break;
        }
        MSS_END();
    }

    // Privates
    void App::print_help_() const
    {
        std::cout << options_.help();
    }

    bool App::collect_info_() const
    {
        MSS_BEGIN(bool);

        auto folder = options_.folder;
        if (!folder)
            folder = std::filesystem::current_path();
        MSS(folder);

        std::uintmax_t total_size{};

        supr::filesystem::EachPath each_path({});
        each_path(*folder, [&](const std::filesystem::path &path) {
            const auto size = std::filesystem::file_size(path);
            total_size += size;
            std::cout << path << ';' << size << std::endl;
        });
        std::cout << "TOTAL SIZE;" << total_size << std::endl;

        MSS_END();
    }

} // namespace supr
