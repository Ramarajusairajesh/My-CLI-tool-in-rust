# My-CLI-tool-in-rust

A simple CLI tool written in rust which help with LLM chatting like chatgpt / gemini/ github copilot !

I didn't provide any API-keys for this script ! So you require your own api-keys to use this too!
I pasted my api-keys in /opt/keys/chatbots_api_keys.txt , you can either follow me and create a file like that and copy paste your apis there 
  **OR**
You can just change the value of the api variables in the script to your own api keys and keep on using my script !

To run this tool you require the following tools

**cargo**

**curl**

If your system doesn't have these then follow these steps to install:

    sudo apt install curl #apt for debian or use pacman/yay for arch based os or use dnf for fedora based os 
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

To create aou file for the script to access api-keys:
    
    sudo mkdir /opt/keys
    sudo chown -R $(whoami):$(whoami) /opt/keys
    sudo vim /opt/keys/chatbots_api_keys.txt

    
To run the tool follow the following steps:

    git clone https://github.com/Ramarajusairajesh/My-CLI-tool-in-rust/
    cd My-CLI-tool-in-rust
    cargo run


 Paste your api keys in the following order:
  1.Gemini-api
  2.Chatgpt-api
  
