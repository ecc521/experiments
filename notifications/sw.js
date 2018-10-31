async function handlefetch(event) {
  console.log(event)
  event.respondWith(fetch(event.request))
}

self.addEventListener("fetch", handlefetch) 
