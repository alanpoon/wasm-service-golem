# wasm-service golemcloud

This example is a golemcloud SSR proof of concept that builds on top of HTMX and Rust for frontend development.

The original example is to use Webassembly (rust) to render HTML strings using service worker. Service worker can intercept the HTMX Ajax example hx-get="/todos", pipes request into the Webassembly's exported functions. The original example, the todolist is stored inside the user browser. Clearing browser cache deletes the todolist. 

Using SSR, will allow us to store the todolist inside the golemcloud, achieving shared state.

## Golemcloud with HTMX

Golemcloud's response is always a JSON with an Array. Htmx requires response to return as HTML string. In order to fix this, service worker can be used again to modify the response from golem cloud. The following code fixes:

```js
if (event.request.url.includes("/golem")) {
        var b = await fetch(event.request);
        if (b.status == 200) {
          var bb = await b.json()
            if (Array.isArray(bb)) {
              if (bb.length > 0) {
                request = JSON.stringify({
                  method: event.request.method,
                  url: event.request.url,
                  headers: Array.from(event.request.headers),
                  body: bb[0],
                });
              }
            }
          
        }
      } else {
          request = JSON.stringify({
          method: event.request.method,
          url: event.request.url,
          headers: Array.from(event.request.headers),
          body: await event.request.text(),
        });     
 }

```
If the htmx's request path contains "/golem", we fetches the response, if the response status is 200 and the body is an array, we take the first element if it exists.

## HTML from Golem
![screenshot](/GolemMe.png)
Clicking the button "Golem me", it will get the HTML from golem cloud, "Hello" in bold.

## Guide to running this project 
- Generate your self-signed certificate using localhost as common name. Perhaps using mkcerts. 
- Put the certs inside "certs" in this format: "server-cert.pem", "server-key.pem"
- Add self-signed generate certs to system or browser

```sh
make
cd .devcontainer
docker compose up -d nginx
```
- go to https://localhost:4003
- click the button "Golem me"

## Future exploration
- Currently there is bug in golem's cloud Gateway API for Request body as parameter
- Expore hx-post with json encoded parameters
- Expore using service worker to convert Post form into Post JSON.