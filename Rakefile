
task :default => [:fmt, :test, :doc, :oobt, :release]

task :fmt do 
  sh("cargo fmt")
end

task :clean do
  sh("cargo clean")
end

task :test do 
  sh("cargo test")
end

task :doc do 
  sh("cargo doc --no-deps --examples")
end

task :oobt do 
  Dir.glob('examples/*.rs').each do |path|
    name = File.basename(path, '.rs')
    ENV['RUST_BACKTRACE'] = '1'
    sh("cargo run --example #{name}")
  end
end

task :release => [:fmt, :test, :oobt] do 
  sh("cargo publish")
end
