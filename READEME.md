# What it is?
A simple webscraping script, that allows me to read contents of our school
canteens website and notify me, when new meals are present.

# What have I learnt?
How to use rust webscraper
How to use enviroment variables in Rust
How to send emails in rust
How to write to a text file in Rust

# Deployment
1. Fork the project and download it to your pc
2. In gmail create a new app for your email and write your email and 
generated password into .env
3. run cargo build in the root of the project
4. in windows task scheduler create a new task that runs the .exe file created in /target/debug every day