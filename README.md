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
4. in windows task scheduler create a new task that runs the .exe file created in /target/debug every day, give it administrator privilages

## Environment Variables

To run this project, you will need to add the following environment variables to your .env file

`EMAIL` = your email through which you can send emails using code [text](https://www.youtube.com/watch?v=JRCJ6RtE3xU&t=599s&ab_channel=CoreySchafer)
`EMAIL_PASSWORD`= password for such an email
`TO_EMAIL` = email to send emails to 