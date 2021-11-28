# mixtape-bot/api

👽️ does things

- **current version:** v1

## votes

- **redis key format:** `{site}.{vote_type}:{bot}.{user}`  
- **vote types:** `test` and `upvote`

### [top.gg](https://top.gg)

view the bot webhook data [**here**](https://docs.top.gg/resources/webhooks/#bot-webhooks)

- **key ttl:** 12 hours (720 minutes)
- **example key:**  `top_gg.{type}:{bot}.{user}`
- **endpoint:** `POST /{version}/voting/top-gg`

---

Copyright &copy; Mixtape Bot 2019 - 2021
