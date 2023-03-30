# ChatWise
Welcome to ChatWise, a command-line interface program designed to hold a conversation with you! ChatWise is powered by OpenAI's GPT language model, which means that you can expect it to have a wide range of knowledge and be able to respond to a variety of topics. Whether you're looking for some entertainment, want to test your knowledge, or simply want to have a chat, ChatWise is here to keep you company. So sit back, relax, and let's chat!


## Set API key via Environment variable
To set an environment variable in a terminal session, you can use the export command followed by the variable name and value. For example, to set an environment variable called API_KEY with a value of 12345, you would enter the following command:
```
export OPENAI_API_KEY=api_key_here
```
after running this command, the `OPENAI_API_KEY` environment variable will be set for the current terminal session. You can verify that the variable is set by running the `env` command, which will display a list of all environment variables currently set in the session.

Note that any child processes launched from the terminal session will inherit the environment variables set in that session. However, once the session is closed, the environment variables will be lost. To set environment variables permanently, you may need to modify your shell startup files, such as `~/.bashrc` or `~/.bash_profile`, `~/.zshrc` etc.


## Building
Assuming you have rust and cargo installed, clone the repository and run the following on the command line

```
$ export OPENAI_API_KEY=api_key_here
$ cargo build

# Run and save the output to a Markdown file
$ cargo run -- --output chat.md
```
<br><br>
Bob Peters<br>
[Rust-Trends.com](https://Rust-Trends.com)