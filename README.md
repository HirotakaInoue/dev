# Docker 

```zsh
docker-compose up
```

# DB
## run migration
```zsh
export DATABASE_URL="postgres://usr:pass@localhost:8081/tmp_db?sslmode=disable"
sqlx migrate run
```

## make migration file
```zsh
sqlx migrate add [file_name]
```
-> build migration file at ./migrations/
