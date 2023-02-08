## Install

### Config

**Meilisearch**

In order for Manucure to use search, a MeiliSearch instance needs to be exposed on a public domain. 

You will also need to generate and API key for the instance. 
You can use [pass](https://www.passwordstore.org/) or any tool capable of generating strong passwords.
Once done fill the `[search_engine]` fields accordingly. 

**OICD client**

You need to configure a client on your Open ID Connect provider. 
Once done fill the `[oauth_provider]` fields with the appropriate values.

**Postgres**
Manucure takes care of the database migration for you. 
The only thing you'll need is to setup a PostgreSQL instance and create a database. 
Please refer to the `[database]` section in the example below. 

**Example**

```shell
port = 3000
debug = true
domain = "manucure.org"

[database]
host = "localhost"
port = 5432
database = "manucure"
user = "postgres"
password = "postgres"

[search_engine]
url = "search.manucure.org"
api_key = "my_api_key"

[oauth_provider]
client_id = "manucure"
client_secret = "xZjWvaWPcHlglqcp5b19iLdke5sf9JG7"
provider = "https://keycloak.cloud.me"
user_info_url = "/auth/realms/me/protocol/openid-connect/userinfo"
auth_url = "/auth/realms/me/protocol/openid-connect/auth"
token_url = "/auth/realms/me/protocol/openid-connect/token"
```

### Docker compose

```shell
version: "3.9"
services:
  manucure:
    image: "oknozor/manucure:latest"
    depends_on:
      - postgres
      - meilisearch
    restart: unless-stopped
    container_name: manucure
    ports:
      - "3000:3000"
      - "2222:22"
    volumes:
      - ./config.toml:/etc/manucure/config.toml
    networks:
      - manucure

  postgres:
    image: docker.io/postgres:13.2
    restart: unless-stopped
    environment:
      POSTGRES_USER: "postgres"
      POSTGRES_PASSWORD: "postgres"
      POSTGRES_DB: "manucure"
    ports:
      - "5432:5432"
    volumes:
      - ./docker/init.sql:/docker-entrypoint-initdb.d/init.sql
    networks:
      - manucure
  meilisearch:
    image: getmeili/meilisearch:v1.0
    restart: unless-stopped
    environment:
      MEILI_MASTER_KEY: "fMuxK9AIppcbb6H08T]gm>YAz"
    ports:
      - "7700:7700"
    volumes:
      - ./docker/meili_data:/meili_data
    networks:
      - manucure
```