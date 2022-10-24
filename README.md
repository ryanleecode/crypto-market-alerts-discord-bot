<h3 align="center">
	Crypto Market Alerts Bot
</h3>
<!-- 
<p align="center">
	<a href="https://demo.thelounge.chat/"><img
		alt="#thelounge IRC channel on Libera.Chat"
		src="https://img.shields.io/badge/Libera.Chat-%23thelounge-415364.svg?colorA=ff9e18"></a>
	<a href="https://yarn.pm/thelounge"><img
		alt="npm version"
		src="https://img.shields.io/npm/v/thelounge.svg?colorA=333a41&maxAge=3600"></a>
	<a href="https://github.com/thelounge/thelounge/actions"><img
		alt="Build Status"
		src="https://github.com/thelounge/thelounge/workflows/Build/badge.svg"></a>
</p> -->

<!-- <p align="center">
	<img src="https://raw.githubusercontent.com/thelounge/thelounge.github.io/master/img/thelounge-screenshot.png" width="550">
</p>
 -->
## Overview

This project is a discord bot that lets you hook up any JSON formatted market alert data and exposes it as a slash (/) command in discord. It is built using [tokio](https://tokio.rs/), an asynchronous runtime for Rust, [Serenity](https://github.com/serenity-rs/serenity), a discord bot framework, [Warp](https://github.com/seanmonstar/warp), a webserver framework, and [Sea-ORM](https://www.sea-ql.org/SeaORM/) a Rust SQL ORM.

Currently this project is under development.

## Supported Commands

`/alerts [category]`

Queries alerts based on category and shows the corresponding tickers line by line by interval.

## Routes

```ts
/POST alerts

{
  ticker: string,
  timestamp: ISOString,
  signal: string,
  category: string,
  interval: string
}
```

## Development setup

```sh
docker-compose up -d # start up database and hashicorp vault
vault kv put -mount=secret cma_bot db_pw=password discord_token="XXXXXXXXXX" # replace with your discord token
export DATABASE_URL="postgres://postgres:password@localhost:6000/postgres"
sea-orm-cli migrate
cargo run "VAULT_TOKEN" "DISCORD_GUILD_iD" # replace with yours. Default vault token can be found in settings/docker-compose
```
