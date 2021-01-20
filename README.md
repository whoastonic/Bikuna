# kuna

yet another HTTP client that's stuck in the terminal...

## Install

```bash
$ cargo install kuna
```

## Usage

```bash
### GET REQUESTS ###

# GET request to URL:
$ kuna http://example.com

# GET request to URL (+ output file):
$ kuna http://example.com -o path/to/file.txt

### POST REQUESTS ###

# POST request to URL (no body):
# provides an empty payload instead
$ kuna -m POST http://example.com

# POST request to URL (+ body):
$ kuna -m POST -p "hello-server?" http://example.com

# POST request to URL (file payload):
$ kuna -m POST -p "/path/to/payload.json" http://example.com

### DELETE REQUESTS ###

# DELETE request to URL:
## takes the same params as POST Requests
$ kuna -m DELETE http://example.com
```

## Support & Goals

If'll like to know where this project is heading, or wanna help out in some way or form... join the [Discord](https://discord.com/invite/PjRbWMH) there you can ask questions about the library and how you can help or recieve help on an issue ;D
