# Image Counter Bot

Discord on mobile gives no indication when a tweet (or other embed) contains multiple images.
This bot listens for messages from users (and background updates from discord's embed backend service) that have multiple embeds with images in them and sends a message accordingly.

To set it up, add your `DISCORD_TOKEN` to a `.env` file in the working directory and run it.

## Known issues
I'm not sure how this bot will work with threads.