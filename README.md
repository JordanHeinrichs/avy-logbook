# Avy Logbook

Work in progress. It will integrate with Strava and link avy conditions to backcountry ski activities for looking over what the conditions were.
Still in the project setup phase.

It will be deployed using CDK to AWS with the following infrastructure:
- Svelte front-end is statically compiled and goes to an S3 bucket
- Rust back-end is built for a lambda
- Cloudfront routes anything in the /api routes to the lambda function URL. Otherwise returns index.html for the SPA
- Authentication is OAuth2 using Strava. Token is put into a JWT and stored in the session.
- Persistance will be done using DynamoDB

# Back end - Rust Axum
- located in `./back_end`
- serves front end directory
- middleware for checking authorization header
- middleware for checking session that user exists
- store example that holds token secret for authorization
- /api route example using authorization header
- /secure route example using sessions for authorization

Note there is no persistance beyond what's held in memory while the application is running

run as `cargo run` from parent directory and not needed to run inside `./back_end` folder

Dynamo DB local setup:
`docker run -p8000:8000 amazon/dynamodb-local:latest`

Write
```bash
curl -X POST http://localhost:9000/hello_world -H 'Content-Type: application/json' -d '{"first_name": "test", "last_name": "foo"}'
```

```bash
aws dynamodb create-table --endpoint-url http://localhost:8000 \
    --table-name AvyLogbook \
    --attribute-definitions \
        AttributeName=PK,AttributeType=S \
        AttributeName=SK,AttributeType=S \
    --key-schema \
        AttributeName=PK,KeyType=HASH \
        AttributeName=SK,KeyType=RANGE \
    --table-class STANDARD \
    --billing-mode PAY_PER_REQUEST
```
To check item:
```
aws dynamodb get-item --endpoint-url http://localhost:8000 --consistent-read \
    --table-name AvyLogbook \
    --key '{ "PK": {"S": "test"}, "SK": {"S": "foo"}}'
```

# Front end - Svelte
- Located in `./front_end`
- navbar with login and logout
- secure page that shows session information once logged in
- api fetch example, log in not required

run as `npm run build` from inside the `./front_end` directory to build the static serve file directory.

# Setup

Install the following
NodeJs - [Install](https://nodejs.org/en/download/)
Rust  - [Install](https://www.rust-lang.org/tools/install)

Clone the repository
- cd repository
- inside the `./front_end` folder run `npm install` to download all module dependencies inside root directory of project
- inside the `./front_end` folder run `npm run build` to bundle the js/svelte code into public folder
- inside the top level folder run `cargo run` to start the the server
- access in browser at http://localhost:8080/
