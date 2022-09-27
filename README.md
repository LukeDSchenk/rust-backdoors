# rust-backdoors

More bind and reverse shells! This time written in Rust. The bind shell acts as a simple multithreaded server dishing out bash sessions, and the reverse shell just opens a connection to wherever you please and hands out a bash shell. In order to compile and use these shells, you will need to [install Rust](https://www.rust-lang.org/tools/install).

## Bind Shell

To use the bind shell, clone down the repo and `cd` into the `bind-shell` directory. For a quick test, you can simply use the `cargo run` command, and your machine will start up a bind shell on port 4444! You can then test that it works using a tool like netcat (try `nc localhost 4444`), or a programming language of your choice. Open up a TCP connection and you will be greeted with a shell session on your local machine:

![Rust bind shell example.](https://i.imgur.com/WwkbYUd.png "Rust bind shell example.")

*Note that `ncat` is just a newer version of the standard `nc`, but is functionally the same here.*

If you would like the shell to listen on a different port, simply swap out port 4444 in the code of `main.rs`. *Also, note that the # of connections thing needs work lol.*

## Reverse Shell

To use the reverse shell, clone down the repo and `cd` into the `reverse-shell` directory. For a quick test, you can set up a simple netcat listener in one terminal (`nc -lvp 4444`) and then execute the program in another terminal (`cargo run`) as shown in these photos:

![Rust reverse shell example.](https://imgur.com/QxRj8dj.png "Rust revese shell example.")

*Make sure that you specify port 4444 if you want it to work right out of the box!*

If you would like the reverse shell to open a connection somewhere else, simply swap out `"localhost:4444"` for a connection string to any other location of your choosing (IP address, hostname, etc.).

## Using these shells in real (ethical) scenarios

I am not going to lecture you about using these shells for mischief, but I urge everyone to be a good person.

If you were going to use these shells in a(n ethical) scenario, you would likely just want to use the compiled binary file, and not the whole Rust project. For those of you who are unfamiliar with Rust, you can compile a release binary really easily using this command:

```bash
cargo build --release
```

So basically you just `cd` into the directory of whichever shell you want to use, modify the code to suit your needs, and then run `cargo build --release`. From there, you can copy the binary file that is created to wherever you want and simply execute it. From start to finish that looks like this:

```bash
git clone https://github.com/LukeDSchenk/rust-backdoors.git

cd rust-backdoors/bind-shell     # or rust-backdoors/reverse-shell

# Modify code as needed

cargo build --release
cp target/release/bind-shell new_dir/new_name  # or target/release/reverse-shell

cd new_dir
./new_name
```

That's it, I wish you the best in your (ethical) endeavors, and I hope these shells help you learn something.
