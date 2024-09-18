use env_perm;

fn main() {
    // Check if DUMMY is set, if not set it to 1
    // export DUMMY=1
    env_perm::check_or_set("DUMMY", 1).expect("Failed to find or set DUMMY");
    // Append $HOME/some/cool/bin to $PATH
    // export PATH= "$HOME/some/cool/bin:$PATH"
    env_perm::append("PATH", "$HOME/some/cool/bin").expect("Couldn't find PATH");
    // Append $HOME/some/cooler/bin to the end of the path
    // export PATH= "$PATH:$HOME/some/cooler/bin"
    env_perm::append_to_end("PATH", "$HOME/some/cooler/bin").expect("Couldn't find PATH");
    // Sets a variable without checking if it exists.
    // Note you need to use a raw string literal to include ""
    // export DUMMY="/something"
    env_perm::set("DUMMY", r#""/something""#).expect("Failed to set DUMMY");
}
