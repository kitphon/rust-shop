# downloads and compiles all dependencies listed
`cargo build`

# start API
`cargo run`
 
# manually run migrations
`cargo run -- migrate`

# generate entity from database
`sea-orm-cli generate entity -o entity/src`

# start database
`docker-compose --profile only-db up --build -d`