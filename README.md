# Split

#### Prerequisites

You will need the usual build essential along with the rust toolchain and docker for the backend. The frontend will need everything needed for react, like node.

#### How to run

The project uses `make`:

| Task Name | Description                                                                             |
| --------- | --------------------------------------------------------------------------------------- |
| all       | Runs any necessary other make targets and deploys the application locally               |
| models    | Generates the source code for the models specified in the OPENAPI specifcations         |
| docs      | Generates the source code for the documentation specified in the OPENAPI specifications |
| compose   | Runs docker compose and removes any dangling images                                     |
| recompose | Runs docker compose, rebuilds the images and removes any dangling images                |
| clean     | Removes any caches and build artifacts                                                  |
| purge     | Removes any caches, build artifacts and volumes (THIS WILL DELETE YOUR DB's DATA!!!)    |

### Planned features

- Add support for users, groups and expenses
- Add simplification of the expenses graph
- Add auth using JWT and ECC
- Add caching and rate limiting through redis
- Add client (prob TS + react)
- Add automatic deployment to RaspPi
- Add automatic generation and deployment of openapi docs through github pages
