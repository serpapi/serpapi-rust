
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
  sh("cargo doc")
end

task :oobt do 
  sh("cargo run --example example_google_search")
end

task :release => [:fmt, :test, :oobt] do 
  sh("cargo publish")
end
