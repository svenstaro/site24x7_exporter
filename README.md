# site24x7_exporter

[![GitHub Actions Workflow](https://github.com/svenstaro/site24x7_exporter/workflows/Build/badge.svg)](https://github.com/svenstaro/site24x7_exporter/actions)
[![Docker Cloud Build Status](https://img.shields.io/docker/cloud/build/svenstaro/site24x7_exporter)](https://cloud.docker.com/repository/docker/svenstaro/site24x7_exporter/)
[![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/svenstaro/site24x7_exporter/blob/master/LICENSE)

A Prometheus compatible exporter for site24x7.com

## Features

This exporter currently supports these monitor types:

- URL "Website"
- HOMEPAGE "Web Page Speed (Browser)"
- REALBROWSER "Web Transaction (Browser)"

It also supports monitor groups and exposes them via tags.

## How to use

### Preparation

First you need to create an OAuth 2.0 application as per https://www.site24x7.com/help/api/index.html#getting-started
For instance, go to https://api-console.zoho.eu (or your whatever your region's endpoint is) and then create a new
application of type "Self Client". Note the client's `Client ID` and `Client Secret`, you'll need them later.

Now it's time to get a refresh token. First, we'll need to generate a temporary code. In the "Generate Code" tab,
enter `Site24x7.Reports.Read` as the scope.
Choose a time duration of 10 minutes for the code and finally click "CREATE". You'll receive a temporary code.

In order to get your permanent refresh token, run this curl:

    curl https://accounts.zoho.eu/oauth/v2/token -X POST \
        -d "client_id=your-client-id" \
        -d "client_secret=your-client-secret" \
        -d "code=your-temporary-code" \
        -d "grant_type=authorization_code"

Note: Remember to use your proper region endpoint!

You'll get back a response that looks roughly like this:

```
{
    "access_token": "some long token ",
    "api_domain": "https://www.zohoapis.eu",
    "expires_in": 3600,
    "refresh_token": "we're interested in whatever is in here",
    "token_type": "Bearer"
}
```

Copy the value of `refresh_token` somewhere, we'll need it later.

### Run via cargo

The exporter expects to receive the OAuth 2.0 data via environment variables.
These are:

- `ZOHO_CLIENT_ID`
- `ZOHO_CLIENT_SECRET`
- `ZOHO_REFRESH_TOKEN`

Let's set them and spin up our exporter:

    export ZOHO_CLIENT_ID=your-client-id
    export ZOHO_CLIENT_SECRET=your-client-secret
    export ZOHO_REFRESH_TOKEN=your-refresh-token
    cargo run -- --site24x7-endpoint site24x7.eu
    curl http://localhost:9803/metrics

Alternatively you can add these environment variables to an `.env` file in this format:

    ZOHO_CLIENT_ID=your-client-id
    ZOHO_CLIENT_SECRET=your-client-secret
    ZOHO_REFRESH_TOKEN=your-refresh-token

This is especially convenient for development purposes.

### Run via docker

    docker run \
        -e ZOHO_CLIENT_ID=your-client-id \
        -e ZOHO_CLIENT_SECRET=your-client-secret \
        -e ZOHO_REFRESH_TOKEN=your-refresh-token \
        svenstaro/site24x7_exporter --site24x7-endpoint site24x7.eu

### Testing

Try

    curl localhost:9803/metrics

and you should see some sweet metrics if everything is working fine.

## Usage in Prometheus

Make sure to not poll this too often as site24x7 has API usage limits per day.
The limit seems to be around 70000 per day so polling every 5 seconds should be safe.
