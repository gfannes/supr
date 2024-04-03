home_dir, gubg_dir = ENV.values_at(*%w[HOME gubg])
here_dir = File.dirname(__FILE__)

task :default do
    sh("rake -T")
end

desc("Install application into '#{gubg_dir}'")
task :install => :build do
    if gubg_dir
        sh("xmake install -v -o #{gubg_dir} supr")
    else
        sh("xmake install --admin supr")
    end
end

desc("Build application")
task :build do
    sh("xmake build -v supr")
end

desc("Generate .clangd file")
task :clangd do
    File.open('.clangd', 'w') do |fo|
        fo.puts("CompileFlags:")
        fo.puts("    Add: [-std=c++20, -I#{File.join(here_dir, 'src')}, -I#{File.join(home_dir, 'decode-it/bugb/src')}]")
    end
end