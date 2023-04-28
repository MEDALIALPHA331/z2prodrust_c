# z2prodrust_c



## Database setup 

* Run docker 
* Pull the postgres image 
* run the init_db script

* create a migration 
    - export DATABASE_URL=postgres://postgres:password@localhost:5432/newsletter
    - sqlx migrate add create_subscriptions_table

* write sql code in the migration
<!-- * run the migration using 
    - sqlx migrate run -->

* open a terminal and run to open the db: 
    - docker exec -it 6e4334d9ca88 psql -U postgres -d newsletter

* Further Migrations 
    - SKIP_DOCKER=true ./scripts/init_db.sh