# Simple analytics server
A minimalist, self-hosted alternative to analytics tools
like Google Analytics.

It does not use any Javascript,
but instead relies on invisible 1 by 1 images
embedded in your website,
whose traffic is logged on a small [Rocket.rs](https://rocket.rs/) server
in a simple [JSON lines](https://rocket.rs/) file.
Additional information is added using CSS.

## Usage
First, you need to set the KEY environment variable
to a secret key.

Then, run the server with `cargo run --release`.
It will be accesible through port 8000.

In your website, embed:

```html
<img src="http://{SERVER URL}/{PAGE NAME}/unknown/image.svg"
     style="display: fixed; width: 0; height: 0;">
```

Where `PAGE NAME` is a custom name which will be logged.
If you want to log which platform is used, use:

```html
<img src="http://{SERVER URL}/{PAGE NAME}/unknown/image.svg"
     srcset="http://{SERVER URL}/{PAGE NAME}/desktop/image.svg 1w, http://{SERVER URL}/{PAGE NAME}/mobile/image.svg 2w"
     sizes="(orientation: portrait) 2px, 1px"
     style="display: fixed; width: 0; height: 0;">
```

Finally, you can log specific parts of your website by embedding
the image with lazy loading in that part.

For example, to learn which users finished an article,
you could create a modified `PAGE NAME` (e.g. adding "-end") and embedding
it in the end of the article:

```html
<img src="http://{SERVER URL}/{PAGE NAME}-end/unknown/image.svg"
     srcset="http://{SERVER URL}/{PAGE NAME}-end/desktop/image.svg 1w, http://{SERVER URL}/{PAGE NAME}-end/mobile/image.svg 2w"
     sizes="(orientation: portrait) 2px, 1px"
     style="width: 1; height: 1; margin: 0"
     loading="lazy">
```

To get the logs from the server,
you can also use the `http://{SERVER URL}/logs` endpoint,
with your secret key as the body of a POST request.

For example, with curl:

```bash
curl "http://{SERVER URL}/logs" -d "{SECRET KEY}"
```

## Example
Use the `website/` directory as an example
running the server locally.
