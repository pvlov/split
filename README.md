# Split

## Overview

Split aims to be a nice way to handle small loans between friends. This repository mainly holds the rust backend and hopefully at some point some
of the client code.

## How to Run

### Create a .env file in the project root and set these variables

```bash
# Postgres
PGUSER=
PGPASSWORD=
PGDATABASE=
PGPORT=
```

#### Run this

```bash
docker compose up --build -d
```
