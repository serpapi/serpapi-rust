
task :default => [:fmt, :test, :doc, :readme, :oobt, :release]

task :fmt do 
  sh("cargo fmt")
end

task :clean do
  sh("cargo clean")
end

task :test do 
  sh("cargo test")
end

task readme: ['README.md.erb'] do
  `erb -T '-' README.md.erb > README.md`
end

task :doc do 
  sh("cargo doc --no-deps --examples")
end

task :oobt do 
  Dir.glob('examples/*.rs').each do |path|
    # filter example broken because of API issues
    if path =~ /google_product/
      puts "skip: #{path}"
      next
    end
    name = File.basename(path, '.rs')
    ENV['RUST_BACKTRACE'] = '1'
    sh("cargo run --example #{name}")
  end
end

task :publish do 
  sh("cargo publish")
end

task :release => [:fmt, :test, :doc, :readme, :oobt, :publish]
