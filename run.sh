docker run -i -t -v /Users/zy/data/rst-schedule:/usr/src -w /usr/src/main rust:1.39.0 /usr/src/target/debug/hello-world











 #target/debug/hello-world
 # docker run -i -t -v $HOME/var/docker:/usr/src -w /mnt/data/helloworld -p 3000:3000 rust:1.39.0 target/hello-world